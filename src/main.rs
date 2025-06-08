use crate::app::GraphtorioApp;

mod app;
mod components;
mod types;
mod views;

pub const GAME_DATA_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "World Study",
        native_options,
        Box::new(|cc| Ok(Box::new(GraphtorioApp::new(cc).unwrap()))),
    )
    .expect("Failed to run egui application.");
}
