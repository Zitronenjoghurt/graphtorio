use crate::app::state::AppState;
use eframe::epaint::Color32;
use egui::Ui;
use egui_snarl::ui::PinInfo;
use egui_snarl::NodeId;
use graphtorio_game::data::GameData;
use graphtorio_game::types::recipe::Recipe;
use graphtorio_game::types::resource::{ResourceIO, ResourceShape};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FactoryViewerState {
    pub current_language: String,
    pub fallback_language: String,
    pub game_data: Arc<GameData>,
    pub dirty_nodes: HashSet<NodeId>,
    pub nodes_to_clear_io: HashSet<NodeId>,
    pub last_update: Instant,
    pub update_interval: Duration,
    pub smelter_options: HashMap<String, Arc<Recipe>>,
}

impl FactoryViewerState {
    pub fn new(app_state: &AppState, update_interval: Duration) -> Self {
        let game_data = app_state.game.data.clone();
        Self {
            current_language: String::new(),
            fallback_language: game_data.default_language.clone(),
            game_data,
            dirty_nodes: HashSet::new(),
            nodes_to_clear_io: HashSet::new(),
            last_update: Instant::now(),
            update_interval,
            smelter_options: HashMap::new(),
        }
    }

    pub fn update(&mut self, app_state: &AppState) {
        if app_state.selected_language != self.current_language {
            self.current_language = app_state.selected_language.clone();
            self.process_smelter_options();
        }
    }

    fn process_smelter_options(&mut self) {
        self.smelter_options = self
            .game_data
            .smelting_recipes
            .iter()
            .map(|recipe| {
                (
                    recipe
                        .name
                        .translate(&self.current_language, &self.fallback_language),
                    Arc::clone(recipe),
                )
            })
            .collect();
    }

    pub fn show_resource_io_label(
        &self,
        ui: &mut Ui,
        io_true: &ResourceIO,
        io_desired: &ResourceIO,
    ) {
        ui.label(format!(
            "{} [{}/{}]",
            io_true
                .resource
                .name
                .translate(&self.current_language, &self.fallback_language),
            io_true.amount,
            io_desired.amount,
        ));
    }

    pub fn build_resource_io_pin(&self, ui: &mut Ui, io: &ResourceIO) -> PinInfo {
        let color = Color32::from_rgb(
            io.resource.color_r,
            io.resource.color_g,
            io.resource.color_b,
        );

        match io.resource.shape {
            ResourceShape::Circle => PinInfo::circle().with_fill(color),
            ResourceShape::Square => PinInfo::square().with_fill(color),
            ResourceShape::Triangle => PinInfo::triangle().with_fill(color),
            ResourceShape::Star => PinInfo::star().with_fill(color),
        }
    }
}
