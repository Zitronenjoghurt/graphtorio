use egui::{Color32, Ui};
use egui_snarl::ui::{PinInfo, SnarlPin, SnarlViewer};
use egui_snarl::{InPin, OutPin, Snarl};
use graphtorio_game::types::node::{Node, NodeTrait};
use graphtorio_game::types::resource::{ResourceIO, ResourceShape};

#[derive(Debug, Default)]
pub struct NodeViewer {
    pub current_language: String,
    pub fallback_language: String,
}

impl NodeViewer {
    pub fn new(current_language: String, fallback_language: String) -> Self {
        Self {
            current_language,
            fallback_language,
        }
    }
}

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        node.title()
    }

    fn inputs(&mut self, node: &Node) -> usize {
        node.inputs()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.input;

        let io = node.input_at_index(pin_index);
        if let Some(io) = io {
            pin_from_resource_io(ui, io, &self.current_language, &self.fallback_language)
        } else {
            PinInfo::default()
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        node.outputs()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let node = &snarl[pin.id.node];
        let pin_index = pin.id.output;

        let io = node.output_at_index(pin_index);
        if let Some(io) = io {
            pin_from_resource_io(ui, io, &self.current_language, &self.fallback_language)
        } else {
            PinInfo::default()
        }
    }
}

fn pin_from_resource_io(
    ui: &mut Ui,
    io: &ResourceIO,
    language: &str,
    fallback_language: &str,
) -> PinInfo {
    ui.label(format!(
        "{} [{}]",
        io.resource.name.translate(language, fallback_language),
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
