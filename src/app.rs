use crate::app::state::AppState;
use crate::views::*;
use crate::GAME_DATA_BYTES;
use eframe::{App, Frame};
use egui::Context;
use egui_snarl::Snarl;
use graphtorio_game::data::parsing::RawGameData;
use graphtorio_game::types::factory_node::FactoryNode;
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
        let game = Game::new(raw_game_data)?;

        let mut factory_nodes = Snarl::new();
        let iron_ore = game.data.find_resource("iron-ore").unwrap().clone();
        let iron_ore_node = FactoryNode::resource(iron_ore, 10);
        let iron_recipe = game.data.find_recipe("iron-smelting").unwrap().clone();
        let iron_smelter = FactoryNode::smelter(Some(iron_recipe));
        let copper_ore = game.data.find_resource("copper-ore").unwrap().clone();
        let copper_ore_node = FactoryNode::resource(copper_ore, 10);
        let copper_recipe = game.data.find_recipe("copper-smelting").unwrap().clone();
        let copper_smelter = FactoryNode::smelter(Some(copper_recipe));
        factory_nodes.insert_node(egui::pos2(000.0, 100.0), iron_ore_node);
        factory_nodes.insert_node(egui::pos2(200.0, 100.0), iron_smelter);
        factory_nodes.insert_node(egui::pos2(400.0, 100.0), copper_ore_node);
        factory_nodes.insert_node(egui::pos2(600.0, 100.0), copper_smelter);

        let selected_language = game.data.default_language.clone();
        let state = AppState {
            game,
            factory_nodes,
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
