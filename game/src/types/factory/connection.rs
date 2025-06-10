use egui_snarl::NodeId;

#[derive(Clone, Copy)]
pub struct NodeInputConnection {
    pub source_node_id: NodeId,
    pub source_pin_index: usize,
    pub target_pin_index: usize,
}

#[derive(Clone, Copy)]
pub struct NodeOutputConnection {
    pub source_pin_index: usize,
    pub target_node_id: NodeId,
    pub target_pin_index: usize,
}
