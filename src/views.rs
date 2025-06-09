use crate::app::state::AppState;
use crate::views::main_menu::MainMenuView;
use egui::Context;

pub mod main_menu;

#[derive(Debug, Default, Copy, Clone)]
pub enum UIView {
    #[default]
    MainMenu,
}

pub trait View {
    fn new(state: &AppState) -> Self;
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

pub struct ViewManager {
    main_menu_view: MainMenuView,
}

impl View for ViewManager {
    fn new(state: &AppState) -> Self {
        Self {
            main_menu_view: MainMenuView::new(state),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view {
            UIView::MainMenu => self.main_menu_view.render(ctx, state),
        }
    }
}
