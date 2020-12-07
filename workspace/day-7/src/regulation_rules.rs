use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RegulationRules {
    pub nodes: HashMap<String, u32>,
    pub vertices: Vec<Vertice>,
    id: u32
}

impl RegulationRules {
    pub fn insert_node(&mut self, name: &str) -> u32 {
        if self.nodes.contains_key(name) {
            self.nodes[name]
        }
        else {
            let new_id = self.id;
            self.nodes.insert(name.to_string(), new_id);
            self.id += 1;
            new_id
        }
    }
}

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct Vertice {
    pub start_id: u32,
    pub end_id: u32,
    pub weight: u32,
}