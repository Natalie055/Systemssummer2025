use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Config {
    worker_threads: usize,
    timeout: Duration,
    max_retries: usize,
    interval_secs: u64, // for periodic monitoring
}

#[derive(Debug, Clone)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
    ssl_valid: Option<bool>,
    body_contains: Option<bool>,
}

fn check_website(client: &Client, url: String, cfg: &Config, tx: Sender<WebsiteStatus>) {
    let mut attempt = 0;
    let mut status = Err("Failed".to_string());
    let mut response_time = Duration::from_secs(0);
    let mut ssl_valid = None;
    let mut body_contains = None;

    while attempt <= cfg.max_retries {
        let start = Instant::now();
        let result = client.get(&url).send();

        match result {
            Ok(resp) => {
                response_time = start.elapsed();
                let code = resp.status().as_u16();

                // SSL check: if HTTPS, connection worked = valid
                ssl_valid = Some(url.starts_with("https://"));

                // Body validation: does it contain "html"?
                let text = resp.text().unwrap_or_default();
                body_contains = Some(text.contains("html"));

                status = Ok(code);
                break;
            }
            Err(e) => {
                response_time = start.elapsed();
                status = Err(e.to_string());
            }
        }

        attempt += 1;
        if attempt <= cfg.max_retries {
            thread::sleep(Duration::from_millis(100));
        }
    }

    let report = WebsiteStatus {
        url,
        status,
        response_time,
        timestamp: Utc::now(),
        ssl_valid,
        body_contains,
    };

    tx.send(report).unwrap();
}

fn monitor_once(urls: Vec<String>, cfg: &Config) -> Vec<WebsiteStatus> {
    let (tx, rx) = mpsc::channel();
    let client = Client::builder()
        .timeout(cfg.timeout)
        .build()
        .unwrap();

    let mut handles = vec![];
    for url in urls {
        let tx_clone = tx.clone();
        let client_clone = client.clone();
        let cfg_clone = cfg.clone();
        let handle = thread::spawn(move || {
            check_website(&client_clone, url, &cfg_clone, tx_clone);
        });
        handles.push(handle);
    }

    drop(tx); // close sender
    let mut results = vec![];
    for received in rx {
        results.push(received);
    }

    for h in handles {
        h.join().unwrap();
    }

    results
}

fn periodic_monitoring(urls: Vec<String>, cfg: Config) {
    loop {
        println!("--- Monitoring cycle at {} ---", Utc::now());
        let results = monitor_once(urls.clone(), &cfg);

        // Print results
        for r in &results {
            println!("{:?}", r);
        }

        // Statistics
        let successes: Vec<_> = results.iter().filter(|r| r.status.is_ok()).collect();
        if !successes.is_empty() {
            let avg_time: Duration = successes.iter().map(|r| r.response_time).sum::<Duration>()
                / (successes.len() as u32);
            println!(
                "Statistics: {} sites up, {} sites down, avg response time {:?}",
                successes.len(),
                results.len() - successes.len(),
                avg_time
            );
        }

        thread::sleep(Duration::from_secs(cfg.interval_secs));
    }
}

fn main() {
    let cfg = Config {
        worker_threads: 10,
        timeout: Duration::from_secs(5),
        max_retries: 1,
        interval_secs: 15, // run every 15 seconds
    };

    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.google.com".to_string(),
        "http://example.com".to_string(),
    ];

    periodic_monitoring(urls, cfg);
}
