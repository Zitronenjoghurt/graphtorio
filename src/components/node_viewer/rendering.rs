use crate::components::node_viewer::state::NodeViewerState;
use egui::Ui;
use egui_snarl::ui::PinInfo;
use graphtorio_game::types::factory_node::{FactoryNode, FactoryNodeTrait};

mod resource;
mod smelter;

pub trait NodeRendering: FactoryNodeTrait {
    fn render_input(
        &self,
        ui: &mut Ui,
        viewer_state: &NodeViewerState,
        pin_index: usize,
    ) -> PinInfo {
        let io = self.input_at_index(pin_index);
        if let Some(io) = io {
            viewer_state.build_resource_io_pin(ui, io)
        } else {
            PinInfo::default()
        }
    }

    fn render_output(
        &self,
        ui: &mut Ui,
        viewer_state: &NodeViewerState,
        pin_index: usize,
    ) -> PinInfo {
        let io = self.output_at_index(pin_index);
        if let Some(io) = io {
            viewer_state.build_resource_io_pin(ui, io)
        } else {
            PinInfo::default()
        }
    }

    fn body_enabled(&self) -> bool {
        false
    }

    fn render_body(&mut self, ui: &mut Ui, viewer_state: &NodeViewerState) {}
}

impl NodeRendering for FactoryNode {
    fn render_input(
        &self,
        ui: &mut Ui,
        viewer_state: &NodeViewerState,
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
        viewer_state: &NodeViewerState,
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

    fn render_body(&mut self, ui: &mut Ui, viewer_state: &NodeViewerState) {
        match self {
            FactoryNode::Resource(resource) => resource.render_body(ui, viewer_state),
            FactoryNode::Smelter(smelter) => smelter.render_body(ui, viewer_state),
        }
    }
}
