// Rust Tutorial for Beginners
// This file contains basic Rust concepts with examples

fn main() {
    // 1. Variables and Basic Types
    println!("\n--- Variables and Types ---");
    
    // Variables are immutable by default
    let name = "Alice";
    println!("Name: {}", name);
    
    // Use 'mut' to make a variable mutable
    let mut age = 25;
    age = 26;
    println!("Age: {}", age);
    
    // Different numeric types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    println!("Integer: {}, Float: {}", integer, float);
    
    // Boolean
    let is_learning = true;
    println!("Learning Rust? {}", is_learning);
    
    // 2. Basic Functions
    println!("\n--- Functions ---");
    greet("World");  // This is how we call the greet function
    bye("John");     // Calling the bye function you created
    let sum = add(5, 3);  // Calling add function and storing its result
    println!("5 + 3 = {}", sum);
    
    // Let's try more function calls with different values
    greet("Alice");
    bye("Bob");
    let another_sum = add(10, 20);
    println!("10 + 20 = {}", another_sum);
    
    // 3. Control Flow
    println!("\n--- Control Flow ---");
    // If-else
    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else {
        println!("{} is greater than or equal to 5", number);
    }
    
    // Loop
    println!("\nCounting to 3:");
    for i in 1..=3 {
        println!("Count: {}", i);
    }

    println!("\nwkwkwkw");
    for i in 3..=5 {
        println!("Count: {}", i);
    }
    
    // 4. Basic Error Handling
    println!("\n--- Error Handling ---");
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(message) => println!("Error: {}", message),
    }
    
    let error_result = divide(10, 0);
    match error_result {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(message) => println!("Error: {}", message),
    }
}

// Function that takes a parameter
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn bye(s: &str) {
    println!("Bye, {}!", s);
}

// Function that returns a value
fn add(a: i32, b: i32) -> i32 {
    a + b  // Note: no semicolon means this is the return value
}

// Function that returns a Result (for error handling)
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
