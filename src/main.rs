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


use std::io::{self, Write};
use serde::Deserialize;

#[derive(Deserialize)]
struct Weather {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

fn main() {
    // Ask the user for their location
    print!("Enter your city: ");
    io::stdout().flush().unwrap();
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();

    // Build the URL for the OpenWeatherMap API
    let api_key = "your-api-key";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city.trim(), api_key);

    // Make an HTTP request to the OpenWeatherMap API
    let response = reqwest::blocking::get(&url).unwrap();
    let body = response.text().unwrap();

    // Parse the JSON response from the API
    let weather: Weather = serde_json::from_str(&body).unwrap();

    // Display the current temperature to the user
    println!("The current temperature in {} is {:.1}°C", city.trim(), weather.main.temp);
}
