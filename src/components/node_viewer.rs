use crate::app::state::AppState;
use egui::{Color32, PopupCloseBehavior, Ui};
use egui_snarl::ui::{PinInfo, SnarlPin, SnarlViewer};
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use graphtorio_game::data::GameData;
use graphtorio_game::types::factory_node::smelter::SmelterNode;
use graphtorio_game::types::factory_node::{FactoryNode, FactoryNodeTrait};
use graphtorio_game::types::recipe::Recipe;
use graphtorio_game::types::resource::{ResourceIO, ResourceShape};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct NodeViewer {
    current_language: String,
    fallback_language: String,
    game_data: Arc<GameData>,
    smelter_options: HashMap<String, Arc<Recipe>>,
}

impl NodeViewer {
    pub fn new(state: &AppState) -> Self {
        let game_data = state.game.data.clone();
        Self {
            current_language: state.selected_language.clone(),
            fallback_language: game_data.default_language.clone(),
            game_data,
            smelter_options: HashMap::new(),
        }
    }

    pub fn update_context(&mut self, language: &str) {
        if language != self.current_language {
            self.current_language = language.to_string();
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
}

impl SnarlViewer<FactoryNode> for NodeViewer {
    fn title(&mut self, node: &FactoryNode) -> String {
        node.title()
    }

    fn inputs(&mut self, node: &FactoryNode) -> usize {
        node.inputs()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.input;

        let io = node.input_at_index(pin_index);
        if let Some(io) = io {
            self.pin_from_resource_io(ui, io)
        } else {
            PinInfo::default()
        }
    }

    fn outputs(&mut self, node: &FactoryNode) -> usize {
        node.outputs()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.output;

        let io = node.output_at_index(pin_index);
        if let Some(io) = io {
            self.pin_from_resource_io(ui, io)
        } else {
            PinInfo::default()
        }
    }

    fn has_body(&mut self, node: &FactoryNode) -> bool {
        match node {
            FactoryNode::Smelter(_) => true,
            _ => false,
        }
    }

    fn show_body(
        &mut self,
        node_id: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) {
        let node = &mut snarl[node_id];

        match node {
            FactoryNode::Smelter(smelter) => self.show_smelter_node(smelter, ui),
            _ => {}
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<FactoryNode>) {
        let out_node = &snarl[from.id.node];
        let out_index = from.id.output;
        let in_node = &snarl[to.id.node];
        let in_index = to.id.input;

        if let Some(out_io) = out_node.output_at_index(out_index) {
            let Some(in_io) = in_node.input_at_index(in_index) else {
                return;
            };

            if !Arc::ptr_eq(&out_io.resource, &in_io.resource) {
                return;
            }

            snarl.connect(from.id, to.id);
        }
    }
}

impl NodeViewer {
    fn pin_from_resource_io(&self, ui: &mut Ui, io: &ResourceIO) -> PinInfo {
        ui.label(format!(
            "{} [{}]",
            io.resource
                .name
                .translate(&self.current_language, &self.fallback_language),
            io.amount
        ));

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

    fn show_smelter_node(&mut self, node: &mut SmelterNode, ui: &mut Ui) {
        ui.vertical(|ui| {
            if let Some(recipe) = node.recipe.clone() {
                ui.horizontal(|ui| {
                    ui.label(
                        recipe
                            .name
                            .translate(&self.current_language, &self.fallback_language),
                    );
                    if ui.small_button("✖").clicked() {
                        node.recipe = None;
                    }
                });
            } else {
                let popup_id = ui.make_persistent_id("recipe_selector_popup");
                let button_response = ui.button("Select Recipe");

                if button_response.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }

                egui::popup_below_widget(
                    ui,
                    popup_id,
                    &button_response,
                    PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(200.0);
                        ui.text_edit_singleline(&mut node.selection_filter);
                        ui.separator();

                        let filtered_options: Vec<_> = self
                            .smelter_options
                            .keys()
                            .filter(|name| {
                                name.to_lowercase()
                                    .contains(&node.selection_filter.to_lowercase())
                            })
                            .collect();

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for option in filtered_options {
                                    if ui.selectable_label(false, option).clicked() {
                                        node.recipe = Some(self.smelter_options[option].clone());
                                        ui.memory_mut(|mem| mem.close_popup());
                                    }
                                }
                            });
                    },
                );
            }
        });
    }
}
