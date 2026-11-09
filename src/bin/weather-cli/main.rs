use std::{env, io};

async fn get_information(api_key: &str, city: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, city
    );
    let resp = reqwest::get(url).await?.error_for_status()?;
    resp.text().await
}

fn get_city() -> String {
    let mut input = String::new();
    println!("Enter the city name:");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read city");
    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env");
    let city = get_city();
    let weather_data = get_information(&api_key, &city).await;

    match weather_data {
        Ok(data) => println!("Weather Data: {}", data),
        Err(e) => eprintln!("Request failed: {}", e),
    }
}