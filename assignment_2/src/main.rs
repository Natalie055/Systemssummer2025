fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut sum = 0;
    let mut current = low;

    while current <= high {
        sum += current;
        current += step;
    }

    *total = sum;
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!();
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words = text.split_whitespace();
    let mut unique_words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for word in words {
        if let Some(pos) = unique_words.iter().position(|&w| w == word) {
            counts[pos] += 1;
        } else {
            unique_words.push(word);
            counts.push(1);
        }
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_word = unique_words[i].to_string();
        }
    }

    (max_word, max_count)
}
