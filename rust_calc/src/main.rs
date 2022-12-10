use std::io::{self, Write};
// Create a struct to hold operation data

// Read using io::stdin().read_line()
pub fn readline(msg: &str) -> i64 {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();
    // Convert to i32
    user_input.trim().parse::<i64>().unwrap()
}
fn main() {
    println!("Calculator in Rust!");
    print!("1. Addition\n2. Subtraction\n3. Multiplication\n4. Division\n");
    let operation = readline("Enter operation: ");
    match operation {
        1 => println!("You chose addition"),
        2 => println!("You chose subtraction"),
        3 => println!("You chose multiplication"),
        4 => println!("You chose division"),
        _ => println!("Invalid operation"),
    }
    let num1 = readline("Enter first number: ");
    let num2 = readline("Enter second number: ");
    match operation {
        1 => println!("{} + {} = {}", num1, num2, num1 + num2),
        2 => println!("{} - {} = {}", num1, num2, num1 - num2),
        3 => println!("{} * {} = {}", num1, num2, num1 * num2),
        4 if num2 != 0 => println!("{} / {} = {}", num1, num2, num1 / num2),
        4 if num2 == 0 => println!("Cannot divide by 0"),
        _ => println!("An error occurred"),
    }
}
