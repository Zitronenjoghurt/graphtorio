use egui::Ui;
use egui_snarl::ui::{PinInfo, SnarlPin, SnarlViewer};
use egui_snarl::{InPin, OutPin, Snarl};
use graphtorio_game::data::GameData;
use graphtorio_game::types::node::{Node, NodeTrait};
use std::sync::Arc;

#[derive(Debug)]
pub struct NodeViewer {
    game_data: Arc<GameData>,
}

impl NodeViewer {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self { game_data }
    }
}

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        node.title(&self.game_data)
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Resource(_) => 0,
            Node::Smelter(_) => 1,
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 150, 200))
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Resource(_) => 1,
            Node::Smelter(_) => 1,
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 150, 100))
    }
}
