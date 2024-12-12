#![windows_subsystem = "windows"]
mod constants;

use constants::{APP_LOGO, WIDGET_HEIGHT, WIDGET_WIDTH, X};
use dotenv::dotenv;
use enums::{Color, Key, Shortcut};
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme};
use frame::Frame;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i64,
    wind: Wind,
    clouds: Clouds,
    dt: i64,
    sys: Sys,
    timezone: i64,
    id: i64,
    name: String,
    cod: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Clouds {
    all: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: f64,
    sea_level: f64,
    grnd_level: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    sys_type: i64,
    id: i64,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    id: i64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct Wind {
    speed: f64,
    deg: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let key = std::env::var("API_KEY").unwrap();

    let mut win = window::Window::new(100, 100, 300, 410, "Weather RS");
    let _theme = ColorTheme::new(color_themes::BLACK_THEME).apply();
    let app_ico = image::SvgImage::from_data(APP_LOGO).ok();
    win.set_icon(app_ico);

    let mut city_lbl = Frame::new(X, 40, WIDGET_WIDTH, WIDGET_HEIGHT, "");
    city_lbl.set_label_size(30);

    let mut city_input = input::Input::new(X, 100, WIDGET_WIDTH, WIDGET_HEIGHT, "");
    city_input.set_label_size(20);

    let mut btn = button::Button::new(X, 150, WIDGET_WIDTH, 60, "Get Weather");
    btn.set_label_size(22);
    btn.set_label_color(Color::from_hex(0xffffff));
    btn.set_color(Color::Green);
    btn.set_shortcut(Shortcut::from_key(Key::Enter));
    btn.set_tooltip("    Enter    ");

    let mut temp_lbl = Frame::new(X, 220, WIDGET_WIDTH, WIDGET_HEIGHT, "");
    temp_lbl.set_label_size(30);

    let mut wind_lbl = Frame::new(X, 260, WIDGET_WIDTH, WIDGET_HEIGHT, "");

    let mut ico_lbl = Frame::new(40, 300, WIDGET_WIDTH, 60, "");
    ico_lbl.set_label_size(60);

    let mut desc_lbl = Frame::new(X, 360, WIDGET_WIDTH, WIDGET_HEIGHT, "");

    win.end();
    win.show();

    btn.set_callback(move |_| {
        let city = city_input.value();
        city_input.set_value("");
        city_lbl.set_label(&city);
        city_input.take_focus().unwrap();

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?&appid={}&q={}&units=metric",
            key, city
        );

        let response = reqwest::blocking::get(url).unwrap();
        let status = response.status();
        match status {
            StatusCode::OK => match response.json::<Response>() {
                Ok(data) => {
                    temp_lbl.set_label(&format!("{}°C ", data.main.temp));
                    wind_lbl.set_label(&format!("{}m/s", data.wind.speed));
                    match data.weather[0].id {
                        800 => ico_lbl.set_label("☀️"),
                        801 => ico_lbl.set_label("🌥️"),
                        300..=321 => ico_lbl.set_label("☂️"),
                        802 | 803 | 804 => ico_lbl.set_label("☁️"),
                        500..=532 => ico_lbl.set_label("🌧️"),
                        200..=232 => ico_lbl.set_label("⛈️"),
                        600..=622 => ico_lbl.set_label("🌨️"),
                        701..=781 => ico_lbl.set_label("🌫️"),
                        _ => {}
                    }
                    desc_lbl.set_label(&format!("{}", data.weather[0].description));
                }
                Err(e) => dialog::alert_default(&e.to_string()),
            },
            StatusCode::NOT_FOUND => dialog::alert_default("City Not Found"),
            StatusCode::BAD_REQUEST => dialog::alert_default("Bad Request.Check the city name"),
            StatusCode::FORBIDDEN => dialog::alert_default("Access denied"),
            StatusCode::BAD_GATEWAY => dialog::alert_default("Bad Gateway"),
            StatusCode::INTERNAL_SERVER_ERROR => dialog::alert_default("Internal Server Error"),
            StatusCode::MOVED_PERMANENTLY => dialog::alert_default("Moved Permanently"),
            _ => dialog::alert_default("HTTP Error"),
        }
    });

    // this code for getting raw string for json-to-rust-structure generator
    //https://app.quicktype.io/
    //let res = reqwest::get(url).await?.text().await?;
    //println!(r"{}", res);

    app.run().unwrap();
    Ok(())
}
