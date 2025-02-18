fn main() {
    
// Solution to question 1
let score = 100;
// score = 200; // This will cause an error because score is immutable
let mut mutable_score = 100;
mutable_score = 200;

// Solution to question 2
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn greet_with_age(name: &str, age: i32) {
    println!("Hello, {}! You are {} years old.", name, age);
}

// Solution to question 3
fn check_temperature(temp: f32) {
    if temp < 0.0 {
        println!("Freezing!");
    } else if temp <= 20.0 {
        println!("Cold");
    } else if temp <= 30.0 {
        println!("Pleasant");
    } else {
        println!("Hot!");
    }
}

// Solution to question 4
// The existing divide function already handles division by zero using Result

// Solution to BONUS Challenge
fn fibonacci(n: u32) -> Result<u32, String> {
    if n <= 0 {
        Err(String::from("Input should be a positive integer"))
    } else if n == 1 {
        Ok(0)
    } else if n == 2 {
        Ok(1)
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 3..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        Ok(b)
    }
}

}