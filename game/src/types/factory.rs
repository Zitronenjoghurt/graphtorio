use crate::types::factory::node::{FactoryNode, FactoryNodeTrait};
use crate::types::recipe::Recipe;
use crate::types::resource::Resource;
use egui::ahash::HashMap;
use egui::Pos2;
use egui_snarl::{InPin, InPinId, NodeId, OutPin, OutPinId, Snarl};
use std::sync::Arc;

pub mod node;

#[derive(Default)]
pub struct Factory {
    pub snarl: Snarl<FactoryNode>,
}

impl Factory {
    /// Recalculates the I/O of a given node and returns all nodes which inputs are connected to this Node's updated outputs.
    pub fn recalculate_io(&mut self, node_id: NodeId) -> Vec<NodeId> {
        let (input_count, output_count) = {
            let Some(node) = self.snarl.get_node(node_id) else {
                return vec![];
            };
            (node.input_count(), node.output_count())
        };

        let in_pins: Vec<InPin> = (0..input_count)
            .map(|in_pin_index| {
                self.snarl.in_pin(InPinId {
                    node: node_id,
                    input: in_pin_index,
                })
            })
            .collect();

        // At input A, there is node B's output C (A, B, C)
        let inputs: Vec<(usize, NodeId, usize)> = in_pins
            .iter()
            .map(|in_pin| {
                in_pin
                    .remotes
                    .iter()
                    .map(|out_pin_id| (in_pin.id.input, out_pin_id.node, out_pin_id.output))
            })
            .flatten()
            .collect();

        let input_amounts: HashMap<usize, u64> = inputs
            .iter()
            .filter_map(|(input, input_node_id, output)| {
                let Some(input_node) = self.snarl.get_node(*input_node_id) else {
                    return None;
                };
                let Some(input_io) = input_node.true_output_at_index(*output) else {
                    return None;
                };

                let input_node_output_pin_id = OutPinId {
                    node: *input_node_id,
                    output: *input,
                };

                let input_node_output_pin = self.snarl.out_pin(input_node_output_pin_id);
                let consumer_node_ids: Vec<NodeId> = input_node_output_pin
                    .remotes
                    .iter()
                    .map(|in_pin_id| in_pin_id.node)
                    .collect();

                let balanced_amount = input_io.amount / consumer_node_ids.len() as u64;

                Some((*input, balanced_amount))
            })
            .fold(HashMap::default(), |mut acc, (input, amount)| {
                *acc.entry(input).or_default() += amount;
                acc
            });

        let out_pins: Vec<OutPin> = (0..output_count)
            .map(|out_pin_index| {
                self.snarl.out_pin(OutPinId {
                    node: node_id,
                    output: out_pin_index,
                })
            })
            .collect();

        let node_mut = self.snarl.get_node_mut(node_id).unwrap();
        node_mut.process_inputs(input_amounts);

        out_pins
            .iter()
            .map(|out_pin| out_pin.remotes.iter().map(|in_pin_id| in_pin_id.node))
            .flatten()
            .collect()
    }

    pub fn clear_node_io(&mut self, node_id: NodeId) -> Vec<NodeId> {
        let (input_count, output_count) = {
            let Some(node) = self.snarl.get_node(node_id) else {
                return vec![];
            };
            (node.input_count(), node.output_count())
        };

        (0..input_count).for_each(|in_pin_index| {
            self.snarl.drop_inputs(InPinId {
                node: node_id,
                input: in_pin_index,
            });
        });

        let dirty_nodes: Vec<NodeId> = (0..output_count)
            .map(|out_pin_index| {
                let out_pin_id = OutPinId {
                    node: node_id,
                    output: out_pin_index,
                };
                let out_pin = self.snarl.out_pin(out_pin_id);
                self.snarl.drop_outputs(out_pin_id);
                out_pin
                    .remotes
                    .iter()
                    .map(|in_pin_id| in_pin_id.node)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        let node_mut = self.snarl.get_node_mut(node_id).unwrap();
        node_mut.on_clear_io();

        dirty_nodes
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
