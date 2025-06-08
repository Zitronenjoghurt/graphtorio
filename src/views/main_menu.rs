use crate::app::GraphtorioApp;
use crate::components::node_viewer::NodeViewer;
use egui::Context;

#[derive(Debug, Default)]
pub struct MainMenuState {
    node_viewer: NodeViewer,
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
