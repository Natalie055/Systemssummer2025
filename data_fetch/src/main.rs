use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Deserialize)]
struct SP500Api {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ResultItem>,
}

#[derive(Debug, Deserialize)]
struct ResultItem {
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
}

#[derive(Debug)]
struct Bitcoin {
    price: Option<f64>,
}

#[derive(Debug)]
struct Ethereum {
    price: Option<f64>,
}

#[derive(Debug)]
struct SP500 {
    price: Option<f64>,
}

fn fetch_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
    let body = ureq::get(url).call()?.into_string()?;
    let parsed = serde_json::from_str(&body)?;
    Ok(parsed)
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Yahoo ticker for Bitcoin in USD
        let resp: SP500Api = fetch_json("https://query2.finance.yahoo.com/v8/finance/chart/BTC-USD")?;
        if let Some(first) = resp.chart.result.first() {
            self.price = Some(first.meta.regular_market_price);
        }
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("bitcoin.txt")?;
            writeln!(file, "{}", price)?;
        }
        Ok(())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Yahoo ticker for Ethereum in USD
        let resp: SP500Api = fetch_json("https://query2.finance.yahoo.com/v8/finance/chart/ETH-USD")?;
        if let Some(first) = resp.chart.result.first() {
            self.price = Some(first.meta.regular_market_price);
        }
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ethereum.txt")?;
            writeln!(file, "{}", price)?;
        }
        Ok(())
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Yahoo ticker for S&P 500
        let resp: SP500Api = fetch_json("https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC")?;
        if let Some(first) = resp.chart.result.first() {
            self.price = Some(first.meta.regular_market_price);
        }
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("sp500.txt")?;
            writeln!(file, "{}", price)?;
        }
        Ok(())
    }
}

fn main() {
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: None }),
        Box::new(Ethereum { price: None }),
        Box::new(SP500 { price: None }),
    ];

    loop {
        for asset in assets.iter_mut() {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Error fetching price: {}", e);
            }
            if let Err(e) = asset.save_to_file() {
                eprintln!("Error saving price: {}", e);
            }
        }
        println!("Fetched and saved all prices. Waiting 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
