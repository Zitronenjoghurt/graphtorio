use crate::app::components::graph::GraphState;
use crate::app::GraphtorioApp;
use egui::Context;

#[derive(Debug, Default)]
pub struct MainMenuState {
    graph: GraphState,
}

pub fn render(ctx: &Context, app: &mut GraphtorioApp) {
    egui::CentralPanel::default().show(ctx, |ui| app.main_menu_state.graph.draw(ui));
}
