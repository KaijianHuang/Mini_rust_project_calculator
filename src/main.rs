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
    println!("Please enter the number of legs:");
    let mut legs = String::new();
    io::stdin()
        .read_line(&mut legs)
        .expect("Failed to read input");
    let legs = legs.trim().parse().expect("Please enter a valid number");

    println!("Please enter the number of heads:");
    let mut heads = String::new();
    io::stdin()
        .read_line(&mut heads)
        .expect("Failed to read input");
    let heads = heads.trim().parse().expect("Please enter a valid number");
    
    for x in 0..=legs/4 {
        for y in 0..=(legs-4*x)/2 {
            if 4*x + 2*y == legs && x+y==heads {
                println!("There are {} rabbits and {} chickens in the cage.", x, y);
                return;
            }
        }
    }
    println!("Invalid number of legs and heads: {} {}", legs, heads);
}
