use crate::components::factory_viewer::state::FactoryViewerState;
use egui::Ui;
use egui_snarl::ui::PinInfo;
use egui_snarl::NodeId;
use graphtorio_game::types::factory::node::{FactoryNode, FactoryNodeTrait};
mod resource;
mod smelter;

pub trait FactoryNodeRenderingTrait: FactoryNodeTrait {
    fn render_input(
        &self,
        ui: &mut Ui,
        viewer_state: &FactoryViewerState,
        pin_index: usize,
    ) -> PinInfo {
        let io_true = self.true_input_at_index(pin_index);
        let io_desired = self.desired_input_at_index(pin_index);
        if let Some(io_true) = io_true {
            if let Some(io_desired) = io_desired {
                viewer_state.show_resource_io_label(ui, io_true, io_desired);
            };
            viewer_state.build_resource_io_pin(ui, io_true)
        } else {
            PinInfo::default()
        }
    }

    fn render_output(
        &self,
        ui: &mut Ui,
        viewer_state: &FactoryViewerState,
        pin_index: usize,
    ) -> PinInfo {
        let io_true = self.true_output_at_index(pin_index);
        let io_desired = self.desired_output_at_index(pin_index);
        if let Some(io_true) = io_true {
            if let Some(io_desired) = io_desired {
                viewer_state.show_resource_io_label(ui, io_true, io_desired);
            };
            viewer_state.build_resource_io_pin(ui, io_true)
        } else {
            PinInfo::default()
        }
    }

    fn body_enabled(&self) -> bool {
        false
    }

    fn render_body(
        &mut self,
        ui: &mut Ui,
        viewer_state: &mut FactoryViewerState,
        node_id: &NodeId,
    ) {
    }
}

impl FactoryNodeRenderingTrait for FactoryNode {
    fn render_input(
        &self,
        ui: &mut Ui,
        viewer_state: &FactoryViewerState,
        pin_index: usize,
    ) -> PinInfo {
        match self {
            FactoryNode::Resource(resource) => resource.render_input(ui, viewer_state, pin_index),
            FactoryNode::Smelter(smelter) => smelter.render_input(ui, viewer_state, pin_index),
        }
    }

    fn render_output(
        &self,
        ui: &mut Ui,
        viewer_state: &FactoryViewerState,
        pin_index: usize,
    ) -> PinInfo {
        match self {
            FactoryNode::Resource(resource) => resource.render_output(ui, viewer_state, pin_index),
            FactoryNode::Smelter(smelter) => smelter.render_output(ui, viewer_state, pin_index),
        }
    }

    fn body_enabled(&self) -> bool {
        match self {
            FactoryNode::Resource(resource) => resource.body_enabled(),
            FactoryNode::Smelter(smelter) => smelter.body_enabled(),
        }
    }

    fn render_body(
        &mut self,
        ui: &mut Ui,
        viewer_state: &mut FactoryViewerState,
        node_id: &NodeId,
    ) {
        match self {
            FactoryNode::Resource(resource) => resource.render_body(ui, viewer_state, node_id),
            FactoryNode::Smelter(smelter) => smelter.render_body(ui, viewer_state, node_id),
        }
    }
}
