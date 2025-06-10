use crate::app::state::AppState;
use crate::components::factory_viewer::FactoryViewer;
use crate::views::View;
use egui::Context;

#[derive(Debug)]
pub struct MainMenuView {
    factory_viewer: FactoryViewer,
}

impl View for MainMenuView {
    fn new(state: &AppState) -> Self {
        Self {
            factory_viewer: FactoryViewer::new(state),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.factory_viewer.update_state(state);
            self.factory_viewer.render_tick(&mut state.game.factory, ui);
        });
    }
}
