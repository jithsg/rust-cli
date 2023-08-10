// src/main.rs
use add_cli::add;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num1: i32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("The first argument is not a valid number!");
        std::process::exit(1);
    });
    let num2: i32 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("The second argument is not a valid number!");
        std::process::exit(1);
    });
    let sum = add(num1, num2);
    println!("{} + {} = {}", num1, num2, sum);
}
