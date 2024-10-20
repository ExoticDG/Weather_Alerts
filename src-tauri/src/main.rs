// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



pub fn get_alerts(lat: f32, lon: f32) -> String {


    let api = format!(
        "https://api.weather.gov/alerts/active.atom?point={lat},{lon}",
        
    );
    reqwest::blocking::get(api, )
        .expect("request failed")
        .text()
        .expect("body failed")
}
// User-Agent: (myweatherapp.com, contact@myweatherapp.com)
fn main() {
    let _usage = format!("Usage: {} [lat] [lon]", std::env::args().next().unwrap());

    let lat = 1234.3465;

    let lon = 1345.23456;

    let body = get_alerts(lat, lon);
    println!("{}", body);
    weather_alerts_lib::run();
}

