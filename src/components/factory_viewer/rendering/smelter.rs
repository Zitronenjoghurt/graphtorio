use crate::components::factory_viewer::rendering::FactoryNodeRenderingTrait;
use crate::components::factory_viewer::state::FactoryViewerState;
use egui::{PopupCloseBehavior, Ui};
use egui_snarl::NodeId;
use graphtorio_game::types::factory::node::smelter::SmelterNode;

impl FactoryNodeRenderingTrait for SmelterNode {
    fn body_enabled(&self) -> bool {
        true
    }

    fn render_body(
        &mut self,
        ui: &mut Ui,
        viewer_state: &mut FactoryViewerState,
        node_id: &NodeId,
    ) {
        ui.vertical(|ui| {
            if let Some(recipe) = self.get_recipe().clone() {
                ui.horizontal(|ui| {
                    ui.label(recipe.get_name(
                        &viewer_state.current_language,
                        &viewer_state.fallback_language,
                    ));
                    if ui.small_button("✖").clicked() {
                        viewer_state.nodes_to_clear_io.insert(node_id.clone());
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
                        ui.text_edit_singleline(self.get_selection_filter_mut());
                        ui.separator();

                        let filtered_options: Vec<_> = viewer_state
                            .smelter_options
                            .keys()
                            .filter(|name| {
                                name.to_lowercase()
                                    .contains(&self.get_selection_filter().to_lowercase())
                            })
                            .collect();

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for option in filtered_options {
                                    if ui.selectable_label(false, option).clicked() {
                                        self.set_recipe(&viewer_state.smelter_options[option]);
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
