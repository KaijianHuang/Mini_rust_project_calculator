use serde::Deserialize;
use std::io::{self, Write};
use reqwest;

#[derive(Deserialize, Debug)]
struct CurrentWeatherResponse {
    current: Option<CurrentWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temp_c: f32,
    condition: Condition,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "1519edbf82994ba980874948231502";
    
    // Take user input for city and country
    print!("Enter the city name: ");
    io::stdout().flush()?;
    let mut city_name = String::new();
    io::stdin().read_line(&mut city_name)?;
    city_name = city_name.trim().to_string();
    
    print!("Enter the country code: ");
    io::stdout().flush()?;
    let mut country_code = String::new();
    io::stdin().read_line(&mut country_code)?;
    country_code = country_code.trim().to_string();

    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={},{}", api_key, city_name, country_code);
    let response = reqwest::blocking::get(&url)?.json::<CurrentWeatherResponse>()?;

    match response.current {
        Some(weather) => {
            println!("The current temperature in {} is {}°C, and the weather condition is {}.", city_name, weather.temp_c, weather.condition.text);
        },
        None => {
            println!("No weather data found for the given location.");
        }
    }

    Ok(())
}
