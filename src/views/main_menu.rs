use crate::app::GraphtorioApp;
use egui::Context;

#[derive(Debug, Default)]
pub struct MainMenuState {}

pub fn render(ctx: &Context, app: &mut GraphtorioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {});
}
