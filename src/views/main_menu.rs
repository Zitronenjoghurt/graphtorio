use crate::app::GraphtorioApp;
use crate::components::node_viewer::NodeViewer;
use egui::Context;

#[derive(Debug)]
pub struct MainMenuState {
    node_viewer: NodeViewer,
}

impl MainMenuState {
    pub fn new(language: String, fallback_language: String) -> Self {
        Self {
            node_viewer: NodeViewer::new(language, fallback_language),
        }
    }
}

pub fn render(ctx: &Context, app: &mut GraphtorioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        app.factory_nodes.show(
            &mut app.main_menu_state.node_viewer,
            &Default::default(),
            1,
            ui,
        );
    });
}
