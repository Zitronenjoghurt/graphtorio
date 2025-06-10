use crate::types::factory::connection::{NodeInputConnection, NodeOutputConnection};
use crate::types::factory::node::{FactoryNode, FactoryNodeTrait};
use crate::types::recipe::Recipe;
use crate::types::resource::{Resource, ResourceIO};
use egui::ahash::HashMap;
use egui::Pos2;
use egui_snarl::{InPin, InPinId, NodeId, OutPin, OutPinId, Snarl};
use std::sync::Arc;

pub mod connection;
pub mod node;

#[derive(Default)]
pub struct Factory {
    snarl: Snarl<FactoryNode>,
}

impl Factory {
    /// Recalculates the I/O of a given node and returns all nodes which inputs are connected to this Node's updated outputs.
    pub fn recalculate_io(&mut self, node_id: NodeId) -> Vec<NodeId> {
        let Some(out_pins) = self.get_node_output_pins(node_id) else {
            return vec![];
        };

        let Some(node_inputs) = self.calculate_node_resource_inputs(node_id) else {
            return vec![];
        };

        let node_mut = self.snarl.get_node_mut(node_id).unwrap();
        node_mut.process_inputs(node_inputs);

        out_pins
            .iter()
            .map(|out_pin| out_pin.remotes.iter().map(|in_pin_id| in_pin_id.node))
            .flatten()
            .collect()
    }

    pub fn clear_node_io(&mut self, node_id: NodeId) -> Vec<NodeId> {
        let dirty_nodes: Vec<NodeId> = self
            .get_node_output_connections(node_id)
            .unwrap_or_default()
            .iter()
            .map(|connection| connection.target_node_id)
            .collect();

        self.drop_node_connections(node_id);

        dirty_nodes
    }

    pub fn get_snarl_mut(&mut self) -> &mut Snarl<FactoryNode> {
        &mut self.snarl
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&FactoryNode> {
        self.snarl.get_node(node_id)
    }

    pub fn get_node_input_count(&self, node_id: NodeId) -> Option<usize> {
        self.get_node(node_id).map(|node| node.input_count())
    }

    pub fn get_node_output_count(&self, node_id: NodeId) -> Option<usize> {
        self.get_node(node_id).map(|node| node.output_count())
    }

    pub fn get_node_input_pin_ids(&self, node_id: NodeId) -> Option<Vec<InPinId>> {
        self.get_node(node_id).map(|node| {
            (0..node.input_count())
                .map(|input_index| InPinId {
                    node: node_id,
                    input: input_index,
                })
                .collect()
        })
    }

    pub fn get_node_input_pins(&self, node_id: NodeId) -> Option<Vec<InPin>> {
        self.get_node(node_id).map(|node| {
            (0..node.input_count())
                .map(|input_index| {
                    self.snarl.in_pin(InPinId {
                        node: node_id,
                        input: input_index,
                    })
                })
                .collect()
        })
    }

    pub fn get_node_output_pin_ids(&self, node_id: NodeId) -> Option<Vec<OutPinId>> {
        self.get_node(node_id).map(|node| {
            (0..node.output_count())
                .map(|output_index| OutPinId {
                    node: node_id,
                    output: output_index,
                })
                .collect()
        })
    }

    pub fn get_node_output_pins(&self, node_id: NodeId) -> Option<Vec<OutPin>> {
        self.get_node(node_id).map(|node| {
            (0..node.output_count())
                .map(|output_index| {
                    self.snarl.out_pin(OutPinId {
                        node: node_id,
                        output: output_index,
                    })
                })
                .collect()
        })
    }

    pub fn get_node_input_connections(&self, node_id: NodeId) -> Option<Vec<NodeInputConnection>> {
        self.get_node_input_pins(node_id).map(|in_pins| {
            in_pins
                .iter()
                .map(|in_pin| {
                    in_pin.remotes.iter().map(|out_pin_id| NodeInputConnection {
                        source_node_id: out_pin_id.node,
                        source_pin_index: out_pin_id.output,
                        target_pin_index: in_pin.id.input,
                    })
                })
                .flatten()
                .collect()
        })
    }

    pub fn get_node_output_connections(
        &self,
        node_id: NodeId,
    ) -> Option<Vec<NodeOutputConnection>> {
        self.get_node_output_pins(node_id).map(|out_pins| {
            out_pins
                .iter()
                .map(|out_pin| {
                    out_pin
                        .remotes
                        .iter()
                        .map(|in_pin_id| NodeOutputConnection {
                            source_pin_index: out_pin.id.output,
                            target_node_id: in_pin_id.node,
                            target_pin_index: in_pin_id.input,
                        })
                })
                .flatten()
                .collect()
        })
    }

    pub fn drop_node_inputs(&mut self, node_id: NodeId) {
        self.get_node_input_pin_ids(node_id)
            .unwrap_or_default()
            .iter()
            .for_each(|in_pin_id| {
                self.snarl.drop_inputs(*in_pin_id);
            });
    }

    pub fn drop_node_outputs(&mut self, node_id: NodeId) {
        self.get_node_output_pin_ids(node_id)
            .unwrap_or_default()
            .iter()
            .for_each(|out_pin_id| {
                self.snarl.drop_outputs(*out_pin_id);
            });
    }

    pub fn drop_node_connections(&mut self, node_id: NodeId) {
        self.drop_node_inputs(node_id);
        self.drop_node_outputs(node_id);
    }

    pub fn calculate_node_resource_inputs(
        &self,
        node_id: NodeId,
    ) -> Option<HashMap<usize, ResourceIO>> {
        self.get_node_input_connections(node_id)
            .map(|input_connections| {
                input_connections
                    .iter()
                    .filter_map(|connection| {
                        let Some(source_node) = self.snarl.get_node(connection.source_node_id)
                        else {
                            return None;
                        };
                        let Some(source_io) =
                            source_node.true_output_at_index(connection.target_pin_index)
                        else {
                            return None;
                        };

                        let source_pin_id = OutPinId {
                            node: connection.source_node_id,
                            output: connection.source_pin_index,
                        };

                        let source_pin = self.snarl.out_pin(source_pin_id);
                        let mut consumer_pin_ids: Vec<InPinId> =
                            source_pin.remotes.iter().copied().collect();
                        consumer_pin_ids.sort_by_key(|pin| (pin.node, pin.input));

                        let consumer_count = consumer_pin_ids.len();
                        if consumer_count == 0 {
                            return None;
                        }

                        let target_position = consumer_pin_ids
                            .iter()
                            .position(|pin| {
                                pin.node == node_id && pin.input == connection.target_pin_index
                            })
                            .unwrap_or(0);

                        let base_amount = source_io.amount / consumer_count as u64;
                        let remainder = source_io.amount % consumer_count as u64;
                        let amount = if (target_position as u64) < remainder {
                            base_amount + 1
                        } else {
                            base_amount
                        };

                        Some((
                            connection.target_pin_index,
                            ResourceIO::new(source_io.resource.clone(), amount),
                        ))
                    })
                    .fold(
                        HashMap::default(),
                        |mut acc: HashMap<usize, ResourceIO>, (target_pin_index, resource_io)| {
                            acc.entry(target_pin_index).or_default().amount += resource_io.amount;
                            acc
                        },
                    )
            })
    }

    pub fn spawn_resource(&mut self, position: Pos2, resource: &Arc<Resource>, amount: u64) {
        let node = FactoryNode::resource(resource.clone(), amount);
        self.snarl.insert_node(position, node);
    }

    pub fn spawn_smelter(&mut self, position: Pos2, optional_recipe: Option<&Arc<Recipe>>) {
        let node = FactoryNode::smelter(optional_recipe.cloned());
        self.snarl.insert_node(position, node);
    }
}
