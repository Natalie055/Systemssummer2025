
const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn main() {

    // Temperature Converter

    let starting_f = 32;

    println!("{}°F = {:.2}°C", starting_f, fahrenheit_to_celsius(starting_f as f64));

 for temp in (starting_f + 1)..=(starting_f + 5) {
        println!("{}°F = {:.2}°C", temp, fahrenheit_to_celsius(temp as f64));
 }

println!();

 // Number Analyzer

 let numbers = [12, 7, 15, 4, 9, 10, 3, 6, 8, 5];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);





    // Guessing game

    println!();

    let secret = 45;
    let mut guesses = 0;

    loop {
        // Simulated user guess (replace with different values to test)
        let guess = 40 + guesses; // just an example increasing guess

        guesses += 1;

        match check_guess(guess, secret) {
            0 => {
                println!("Correct! The secret number was {}.", secret);
                break;
            }
            1 => println!("{} is too high.", guess),
            -1 => println!("{} is too low.", guess),
            _ => (),
        }
    }

    println!("It took you {} guesses.", guesses);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}