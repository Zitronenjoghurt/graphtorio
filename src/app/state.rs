use crate::views::UIView;
use graphtorio_game::Game;

pub struct AppState {
    pub game: Game,
    pub selected_language: String,
    pub current_view: UIView,
}
