#Rust WeatherAPI Program

This program is a simple Rust program that fetches the current weather data for a specified location using the WeatherAPI service. The user inputs the city name and country code, and the program makes a request to the WeatherAPI service to get the current weather data for that location.

Prerequisites
To run this program, you need to have Rust and Cargo installed on your system. You also need to sign up for a free API key on the WeatherAPI website in order to use their service.

Usage
Clone this repository to your local machine.

In a terminal window, navigate to the cloned directory.

Build the program by running the following command:

Copy code
cargo build
Run the program by running the following command:

Copy code
cargo run
When prompted, enter the name of the city and the country code for which you want to get the weather data. For example:

yaml
Copy code
Enter the city name: New York
Enter the country code: US
The program will then make a request to the WeatherAPI service and print out the current temperature and weather condition for the specified location. If there is no weather data available for the specified location, the program will print out an error message.

Contributing
If you find a bug or have a feature request, please open an issue on the GitHub repository. If you would like to contribute to the project, please fork the repository, make your changes, and submit a pull request.
