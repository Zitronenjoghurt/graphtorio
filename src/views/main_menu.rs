use crate::app::state::AppState;
use crate::components::node_viewer::NodeViewer;
use crate::views::View;
use egui::Context;

#[derive(Debug)]
pub struct MainMenuView {
    node_viewer: NodeViewer,
}

impl View for MainMenuView {
    fn new(state: &AppState) -> Self {
        Self {
            node_viewer: NodeViewer::new(state),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.node_viewer.update(state);
            state
                .factory_nodes
                .show(&mut self.node_viewer, &Default::default(), 1, ui);
        });
    }
}
