use crate::app::GraphtorioApp;

mod app;
mod types;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "World Study",
        native_options,
        Box::new(|cc| Ok(Box::new(GraphtorioApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
