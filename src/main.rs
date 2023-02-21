use std::io;

fn main() {
    loop {
        println!("Enter an expression to calculate or q to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "q" {
            break;
        }

        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 3 {
            println!("Invalid input: Please enter an expression with two numbers and an operator separated by spaces.");
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input: {} is not a number", parts[0]);
                continue;
            }
        };

        let op = parts[1];
        let b: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input: {} is not a number", parts[2]);
                continue;
            }
        };

        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => {
                println!("Invalid input: {} is not a valid operator", op);
                continue;
            }
        };
        println!("Result: {}", result);
    }
}
