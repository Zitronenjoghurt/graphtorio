use crate::app::views::main_menu::MainMenuState;
use crate::app::views::*;
use eframe::{App, Frame};
use egui::Context;

mod components;
mod views;

#[derive(Debug, Default)]
pub struct GraphtorioApp {
    current_view: UIView,

    main_menu_state: MainMenuState,
}

impl GraphtorioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for GraphtorioApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.current_view {
            UIView::MainMenu => main_menu::render(ctx, self),
        }
    }
}
