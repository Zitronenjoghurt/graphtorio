use crate::types::factory::node::FactoryNodeTrait;
use crate::types::recipe::Recipe;
use crate::types::resource::ResourceIO;
use egui::ahash::HashMap;
use std::sync::Arc;

pub struct SmelterNode {
    recipe: Option<Arc<Recipe>>,
    selection_filter: String,
    true_input: Vec<ResourceIO>,
    true_output: Vec<ResourceIO>,
}

impl SmelterNode {
    pub fn new(recipe: Option<Arc<Recipe>>) -> Self {
        let true_input = recipe
            .clone()
            .map(|recipe| recipe.get_empty_inputs())
            .unwrap_or_default();

        let true_output = recipe
            .clone()
            .map(|recipe| recipe.get_empty_inputs())
            .unwrap_or_default();

        Self {
            true_input,
            true_output,
            recipe,
            selection_filter: String::new(),
        }
    }

    pub fn get_recipe(&self) -> &Option<Arc<Recipe>> {
        &self.recipe
    }

    pub fn set_recipe(&mut self, recipe: &Arc<Recipe>) {
        self.recipe = Some(recipe.clone());
        self.true_input = recipe.get_empty_inputs();
        self.true_output = recipe.get_empty_outputs();
    }

    pub fn reset_recipe(&mut self) {
        self.recipe = None;
        self.true_input = Vec::new();
        self.true_output = Vec::new();
    }

    pub fn get_selection_filter(&self) -> &String {
        &self.selection_filter
    }

    pub fn get_selection_filter_mut(&mut self) -> &mut String {
        &mut self.selection_filter
    }

    pub fn set_selection_filter(&mut self, filter: String) {
        self.selection_filter = filter;
    }
}

impl FactoryNodeTrait for SmelterNode {
    fn title(&self) -> String {
        "Smelter".to_string()
    }

    fn input_count(&self) -> usize {
        if let Some(recipe) = &self.recipe {
            recipe.inputs.len()
        } else {
            0
        }
    }

    fn output_count(&self) -> usize {
        if let Some(recipe) = &self.recipe {
            recipe.outputs.len()
        } else {
            0
        }
    }

    fn true_input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        self.true_input.get(index)
    }

    fn desired_input_at_index(&self, index: usize) -> Option<&ResourceIO> {
        if let Some(recipe) = &self.recipe {
            recipe.inputs.get(index)
        } else {
            None
        }
    }

    fn true_output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        self.true_output.get(index)
    }

    fn desired_output_at_index(&self, index: usize) -> Option<&ResourceIO> {
        if let Some(recipe) = &self.recipe {
            recipe.outputs.get(index)
        } else {
            None
        }
    }

    fn process_inputs(&mut self, inputs: HashMap<usize, u64>) {
        let Some(recipe) = &self.recipe else { return };

        self.true_input
            .iter_mut()
            .enumerate()
            .for_each(|(index, io)| {
                let amount = inputs.get(&index).unwrap_or(&0);
                io.amount = *amount;
            });
        self.true_output = recipe.calculate_outputs(&self.true_input);
    }

    fn on_clear_io(&mut self) {
        self.reset_recipe();
    }
}
