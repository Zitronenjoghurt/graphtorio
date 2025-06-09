use crate::views::UIView;
use egui_snarl::Snarl;
use graphtorio_game::types::factory_node::FactoryNode;
use graphtorio_game::Game;

pub struct AppState {
    pub game: Game,
    pub factory_nodes: Snarl<FactoryNode>,
    pub selected_language: String,
    pub current_view: UIView,
}
