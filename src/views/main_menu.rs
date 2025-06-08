use crate::app::GraphtorioApp;
use crate::components::node_viewer::NodeViewer;
use egui::Context;
use graphtorio_game::Game;

#[derive(Debug)]
pub struct MainMenuState {
    node_viewer: NodeViewer,
}

impl MainMenuState {
    pub fn new(game: &Game) -> Self {
        let game_data = game.data.clone();
        Self {
            node_viewer: NodeViewer::new(game_data),
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
