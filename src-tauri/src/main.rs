// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// wxrs/src/bin/ch3.rs
pub fn get_air_pollution(lat: f32, lon: f32) -> String {
    let location = "WI";


    let url = format!(
        "https://api.weather.gov/alerts/active.atom?area={}",
        location
    );
    reqwest::blocking::get(url)
        .expect("request failed")
        .text()
        .expect("body failed")
}
// User-Agent: (myweatherapp.com, contact@myweatherapp.com)
fn main() {
    let usage = format!("Usage: {} [lat] [lon]", std::env::args().next().unwrap());

    let lat = std::env::args()
        .nth(1)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);

    let lon = std::env::args()
        .nth(2)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);

    let body = get_air_pollution(lat, lon);
    println!("{}", body);
    weather_alerts_lib::run();
}

