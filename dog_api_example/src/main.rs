fn capture_modify_environment() {
    let mut result = 0;

    let mut calculator = |x, y| { result = x + y };
    calculator(1, 2);
    println!("{}", result);  // Output: 3
    
    // Using FnMut trait
    let mut calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| { result = x + y });
    calculator(1, 2);
    drop(calculator);
    println!("{}", result);  // Output: 3
}