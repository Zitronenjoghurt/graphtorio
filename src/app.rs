use crate::views::main_menu::MainMenuState;
use crate::views::*;
use crate::GAME_DATA_BYTES;
use eframe::{App, Frame};
use egui::Context;
use graphtorio_game::data::parsing::RawGameData;
use graphtorio_game::Game;
use std::error::Error;

#[derive(Debug)]
pub struct GraphtorioApp {
    game: Game,
    current_view: UIView,

    main_menu_state: MainMenuState,
}

impl GraphtorioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn Error>> {
        let decompressed_game_data = zstd::decode_all(GAME_DATA_BYTES)?;
        let raw_game_data = RawGameData::decode(&decompressed_game_data)?;
        let game = Game::new(raw_game_data)?;

        let app = Self {
            game,
            current_view: UIView::MainMenu,
            main_menu_state: MainMenuState::default(),
        };

        Ok(app)
    }
}

impl App for GraphtorioApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.current_view {
            UIView::MainMenu => main_menu::render(ctx, self),
        }
    }
}
