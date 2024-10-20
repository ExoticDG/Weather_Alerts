// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



pub fn get_alerts(lat: f32, lon: f32) -> String {


    // https://api.weather.gov/alerts/active?point=34.3,-104.296

    let api = format!(
        "https://api.weather.gov/alerts/active?point={lat},{lon}",
        
    );

    println!("{api}");

    // reqwest::blocking::get(api)
    //     .header("User-Agent","wallofthrones")
    //     .expect("request failed")
    //     .text()
    //     .expect("body failed")

    let client = reqwest::blocking::Client::new();
    let resp = client.get(api).header("User-Agent","wallofthrones").send();
    resp.expect("didn't work bro").text().expect("nope")
    
}
// User-Agent: (myweatherapp.com, contact@myweatherapp.com)
fn main() {

  

    
    
    let _usage = format!("Usage: {} [lat] [lon]", std::env::args().next().unwrap());

    let lat = 34.3;

    let lon = -104.296;

    let body = get_alerts(lat, lon);
    println!("{}", body);
    weather_alerts_lib::run();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

