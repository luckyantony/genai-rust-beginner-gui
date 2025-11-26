use eframe::{egui, App, NativeOptions};
use egui::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Weather {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f32,
    weather_code: i32,
}

fn main() -> Result<(), eframe::Error> {
    // This fixes the Wayland crash
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 640.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Weather & Dad Jokes • Rust GUI",
        options,
        Box::new(|cc| {
            let mut app = MyApp::default();
            app.ctx = Some(cc.egui_ctx.clone());
            Ok(Box::new(app))
        }),
    )
}

struct MyApp {
    city: String,
    fetching: bool,
    ctx: Option<egui::Context>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            city: "Nairobi".to_string(),
            fetching: false,
            ctx: None,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.ctx = Some(ctx.clone());

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading("Weather + Dad Jokes");
                ui.label("Pure Rust • egui • Moringa Capstone 2025");
                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.label("City: ");
                    ui.text_edit_singleline(&mut self.city);
                });

                let enter_pressed = ctx.input(|i| i.key_pressed(Key::Enter));
                if ui.button("Get Weather").clicked()
                    || (enter_pressed && !self.city.trim().is_empty())
                {
                    if self.fetching { return; }

                    let city = self.city.trim().to_string();
                    self.fetching = true;

                    let ctx = ctx.clone();
                    std::thread::spawn(move || {
                        let result = fetch_weather(&city);
                        let text = result.unwrap_or_else(|e| format!("Error: {e}"));

                        ctx.memory_mut(|mem| mem.data.insert_temp("result".into(), text));
                        ctx.request_repaint();
                    });
                }

                if self.fetching {
                    ui.add_space(20.0);
                    ui.spinner();
                    ui.label("Fetching weather...");
                }

                if let Some(text) = ctx.memory(|m| m.data.get_temp::<String>("result".into())) {
                    ui.add_space(20.0);
                    if text.starts_with("Error") || text.contains("not found") {
                        ui.colored_label(Color32::RED, &text);
                    } else {
                        ui.label(RichText::new(&text).size(36.0).strong());
                    }
                    self.fetching = false;
                }

                ui.add_space(40.0);
                ui.separator();
                ui.add_space(20.0);
                ui.heading("Dad Joke of the Moment");

                let jokes = [
                    "Why don't skeletons fight each other? They don't have the guts.",
                    "What do you call fake spaghetti? An impasta.",
                    "Why did the scarecrow win an award? He was outstanding in his field!",
                    "How do you organize a space party? You planet.",
                    "I told my wife her eyebrows were too high. She looked surprised.",
                    "Why don't eggs tell jokes? They'd crack each other up.",
                    "What do you call cheese that isn't yours? Nacho cheese!",
                    "Why don't programmers like nature? It has too many bugs.",
                ];

                static JOKE_INDEX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

                if ui.button("New Joke").clicked() {
                    let next = (JOKE_INDEX.load(std::sync::atomic::Ordering::Relaxed) + 1) % jokes.len();
                    JOKE_INDEX.store(next, std::sync::atomic::Ordering::Relaxed);
                }

                let idx = JOKE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
                ui.add_space(10.0);
                ui.label(RichText::new(jokes[idx]).italics().size(22.0));

                ui.add_space(40.0);
                ui.label("Made with Rust • Luckyantony Leshan");
            });
        });
    }
}

// BLOCKING VERSION — NO async, NO tokio, NO pollster needed
fn fetch_weather(city: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en",
        urlencoding::encode(city)
    );

    let resp: serde_json::Value = reqwest::blocking::get(&geo_url)?.json()?;

    let results = resp["results"].as_array().ok_or("City not found")?;
    if results.is_empty() {
        return Err("City not found".into());
    }

    let lat = results[0]["latitude"].as_f64().ok_or("Missing latitude")?;
    let lon = results[0]["longitude"].as_f64().ok_or("Missing longitude")?;

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,weather_code"
    );

    let data: Weather = reqwest::blocking::get(&weather_url)?.json()?;
    let description = code_to_text(data.current.weather_code);

    Ok(format!("{:.1}°C • {description}", data.current.temperature_2m))
}

fn code_to_text(code: i32) -> &'static str {
    match code {
        0 => "Clear sky",
        1..=3 => "Mainly clear, partly cloudy",
        45 | 48 => "Fog",
        51..=67 => "Drizzle / Rain",
        71..=77 => "Snow",
        80..=82 => "Rain showers",
        85..=86 => "Snow showers",
        95..=99 => "Thunderstorm",
        _ => "Cloudy",
    }
}