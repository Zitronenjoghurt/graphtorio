use crate::views::main_menu::MainMenuState;
use crate::views::*;
use crate::GAME_DATA_BYTES;
use eframe::{App, Frame};
use egui::Context;
use egui_snarl::Snarl;
use graphtorio_game::data::parsing::RawGameData;
use graphtorio_game::types::node::Node;
use graphtorio_game::Game;
use std::error::Error;

pub struct GraphtorioApp {
    pub game: Game,
    pub factory_nodes: Snarl<Node>,

    pub current_view: UIView,
    pub main_menu_state: MainMenuState,
}

impl GraphtorioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn Error>> {
        let decompressed_game_data = zstd::decode_all(GAME_DATA_BYTES)?;
        let raw_game_data = RawGameData::decode(&decompressed_game_data)?;
        let game = Game::new(raw_game_data)?;

        let mut factory_nodes = Snarl::new();
        let iron_ore = game.data.find_resource("iron-ore").unwrap().clone();
        let iron_ore_node = Node::resource_node(iron_ore, 10);
        let iron_recipe = game.data.find_recipe("iron-smelting").unwrap().clone();
        let iron_smelter = Node::smelter_node(iron_recipe);
        factory_nodes.insert_node(egui::pos2(100.0, 100.0), iron_ore_node);
        factory_nodes.insert_node(egui::pos2(300.0, 100.0), iron_smelter);

        let app = Self {
            game,
            factory_nodes,
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
