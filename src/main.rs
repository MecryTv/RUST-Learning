mod env;
use reqwest::blocking::get;
use serde_json::{Value, from_str};
use std::io;

fn main() {
    println!("Welcome. In this application, you can see your City Weather\n");

    println!("Please enter your city name: \n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let city = input.trim();
    let api_key = env::API_KEY;
    let url = format!("http://api.weatherapi.com/v1/current.json?key={api_key}&q={city}");

    println!("\nFetching weather data for {city}...\n");

    let response = get(url).unwrap();
    let body = response.text().unwrap();
    let json: Value = from_str(&body).expect("JSON was not well-formatted");

    if let Some(error) = json.get("error") {
        let message = &error["message"];
        println!("Error fetching weather data: {message}");
        return;
    }

    let timezone = &json["location"]["tz_id"];
    let region = &json["location"]["region"];
    let country = &json["location"]["country"];
    let temp_c = &json["current"]["temp_c"];

    println!("The Weather in {city} is:\n");
    println!("Timezone: {timezone}");
    println!("Region: {region}");
    println!("Country: {country}");
    println!("Temperature: {temp_c}°C");

}