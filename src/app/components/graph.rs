use egui::Ui;
use egui_graphs::{DefaultNodeShape, Graph, GraphView, SettingsInteraction, SettingsNavigation};
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::DefaultIx;
use petgraph::Directed;

#[derive(Debug)]
pub struct GraphState {
    graph: Graph<(), (), Directed, DefaultIx, DefaultNodeShape>,
}

impl Default for GraphState {
    fn default() -> Self {
        let mut g = StableGraph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, ());
        g.add_edge(b, c, ());
        g.add_edge(c, a, ());

        Self {
            graph: Graph::from(&g),
        }
    }
}

impl GraphState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        ui.add(
            &mut GraphView::<(), (), Directed, DefaultIx, DefaultNodeShape>::new(&mut self.graph)
                .with_interactions(
                    &SettingsInteraction::new()
                        .with_dragging_enabled(true)
                        .with_node_clicking_enabled(true)
                        .with_node_selection_enabled(true)
                        .with_edge_clicking_enabled(true)
                        .with_edge_selection_enabled(true),
                )
                .with_navigations(
                    &SettingsNavigation::new()
                        .with_fit_to_screen_enabled(false)
                        .with_zoom_and_pan_enabled(true),
                ),
        );
    }
}
