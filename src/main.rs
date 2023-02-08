//A command-line tool that plays Marco Polo
use clap::Parser;
#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
    Marco {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
// hello::marco_polo(&name)


use std::io;

fn main() {
    fn fibonacci(n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        a
    }

    println!("Enter the number n:");
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: u32 = n_str.trim().parse().unwrap();
    println!("The {}-th Fibonacci number is {}", n, fibonacci(n));
}
