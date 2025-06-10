use crate::app::state::AppState;
use crate::components::factory_viewer::rendering::FactoryNodeRenderingTrait;
use crate::components::factory_viewer::state::FactoryViewerState;
use egui::Ui;
use egui_snarl::ui::{SnarlPin, SnarlStyle, SnarlViewer};
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use graphtorio_game::types::factory::node::{FactoryNode, FactoryNodeTrait};
use graphtorio_game::types::factory::Factory;
use std::sync::Arc;
use std::time::{Duration, Instant};

mod rendering;
mod state;

#[derive(Debug)]
pub struct FactoryViewer {
    state: FactoryViewerState,
}

impl FactoryViewer {
    pub fn new(app_state: &AppState, update_interval: Duration) -> Self {
        Self {
            state: FactoryViewerState::new(app_state, update_interval),
        }
    }

    pub fn update_state(&mut self, app_state: &AppState) {
        self.state.update(app_state);
    }

    pub fn render_tick(&mut self, factory: &mut Factory, ui: &mut Ui) {
        factory
            .get_snarl_mut()
            .show(self, &SnarlStyle::default(), 1, ui);

        if self.state.last_update.elapsed() > self.state.update_interval {
            self.update_factory(factory);
            self.state.last_update = Instant::now();
        }
    }

    fn update_factory(&mut self, factory: &mut Factory) {
        let cleared_dirty_nodes: Vec<NodeId> = self
            .state
            .nodes_to_clear_io
            .drain()
            .map(|node_id| factory.clear_node_io(node_id))
            .flatten()
            .collect();
        self.state.dirty_nodes.extend(cleared_dirty_nodes);

        self.state.dirty_nodes = self
            .state
            .dirty_nodes
            .drain()
            .map(|node_id| factory.recalculate_io(node_id))
            .flatten()
            .collect();
    }
}

impl SnarlViewer<FactoryNode> for FactoryViewer {
    fn title(&mut self, node: &FactoryNode) -> String {
        node.title()
    }

    fn inputs(&mut self, node: &FactoryNode) -> usize {
        node.input_count()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.input;
        node.render_input(ui, &self.state, pin_index)
    }

    fn outputs(&mut self, node: &FactoryNode) -> usize {
        node.output_count()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.output;
        node.render_output(ui, &self.state, pin_index)
    }

    fn has_body(&mut self, node: &FactoryNode) -> bool {
        node.body_enabled()
    }

    fn show_body(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<FactoryNode>,
    ) {
        let node = &mut snarl[node_id];
        node.render_body(ui, &mut self.state, &node_id);
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<FactoryNode>) {
        let out_node_id = from.id.node;
        let out_node = &snarl[out_node_id];
        let out_index = from.id.output;
        let in_node_id = to.id.node;
        let in_node = &snarl[in_node_id];
        let in_index = to.id.input;

        if let Some(out_io) = out_node.desired_output_at_index(out_index) {
            let Some(in_io) = in_node.desired_input_at_index(in_index) else {
                return;
            };

            if !Arc::ptr_eq(&out_io.resource, &in_io.resource) {
                return;
            }

            snarl.connect(from.id, to.id);
            self.state.dirty_nodes.insert(out_node_id);
        }
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<FactoryNode>) {
        snarl.disconnect(from.id, to.id);
        self.state.dirty_nodes.insert(to.id.node);
        self.state.dirty_nodes.insert(from.id.node);
    }
}
