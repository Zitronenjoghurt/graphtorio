use crate::app::state::AppState;
use crate::views::*;
use crate::GAME_DATA_BYTES;
use eframe::{App, Frame};
use egui::Context;
use graphtorio_game::data::parsing::RawGameData;
use graphtorio_game::Game;
use std::error::Error;

pub mod state;

pub struct GraphtorioApp {
    state: AppState,
    view_manager: ViewManager,
}

impl GraphtorioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn Error>> {
        let decompressed_game_data = zstd::decode_all(GAME_DATA_BYTES)?;
        let raw_game_data = RawGameData::decode(&decompressed_game_data)?;
        let mut game = Game::new(raw_game_data)?;

        game.spawn_resource(egui::pos2(100.0, 100.0), "iron-ore", 10);
        game.spawn_resource(egui::pos2(100.0, 200.0), "iron-ore", 10);
        game.spawn_resource(egui::pos2(400.0, 100.0), "copper-ore", 10);
        game.spawn_smelter(egui::pos2(200.0, 100.0), Some("iron-smelting"));
        game.spawn_smelter(egui::pos2(400.0, 200.0), Some("iron-loop"));
        game.spawn_smelter(egui::pos2(400.0, 200.0), Some("iron-loop"));
        game.spawn_smelter(egui::pos2(400.0, 200.0), Some("iron-loop"));
        game.spawn_smelter(egui::pos2(600.0, 100.0), Some("copper-smelting"));

        let selected_language = game.data.default_language.clone();
        let state = AppState {
            game,
            selected_language,
            current_view: UIView::MainMenu,
        };

        let view_manager = ViewManager::new(&state);

        let app = Self {
            state,
            view_manager,
        };

        Ok(app)
    }
}

impl App for GraphtorioApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.view_manager.render(ctx, &mut self.state);
    }
}
