use std::fs;
use crate::regulation_rules::{RegulationRules, Vertice};

pub fn parse_text_file(file_name: String) -> RegulationRules {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> RegulationRules {
    let mut rules : RegulationRules = Default::default();

    content
        // Split each line
        .split("bags.\n")
        .flat_map(|line|
            line.split("bag.\n")
        )
        .filter(|line| { !line.is_empty() })
        .for_each(|line| {
            let bag_and_contains : Vec<&str> = line.split(" bags contain ").collect();

            // Insert if not exist
            let node_id = rules.insert_node(bag_and_contains[0].trim());

            // Parse the second part
            // Handle the "no other bags" case
            match bag_and_contains[1].trim() {
                "no other" => {},
                _ => {
                    bag_and_contains[1].split("bag,")
                        .flat_map(|line| {
                            line.split("bags,")
                        })
                        .for_each(|contained| {
                            // Trim and split on the first whitespace
                            let trimmed = contained.trim();
                            let count_and_contained_name : Vec<&str> = trimmed.splitn(2, " ").collect();

                            let contained_node_id = rules.insert_node(count_and_contained_name[1].trim());
                            rules.vertices.push(Vertice {
                                start_id: node_id,
                                end_id: contained_node_id,
                                weight: count_and_contained_name[0].parse().expect(format!("error while parsing count {}", count_and_contained_name[0]).as_str()),
                            });
                    });
                }
            }
        });
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
".to_string();
        let rules = parse_string(content);

        assert_eq!(rules.nodes["light red"], 0);
        assert_eq!(rules.nodes["bright white"], 1);
        assert_eq!(rules.nodes["muted yellow"], 2);
        assert_eq!(rules.nodes["dark orange"], 3);
        assert_eq!(rules.nodes["shiny gold"], 4);
        assert_eq!(rules.nodes["faded blue"], 5);
        assert_eq!(rules.nodes["dark olive"], 6);
        assert_eq!(rules.nodes["vibrant plum"], 7);
        assert_eq!(rules.nodes["dotted black"], 8);
        assert_eq!(rules.nodes.len(), 9);

        assert!(rules.vertices.contains(&Vertice { start_id: 0, end_id: 1, weight: 1 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 0, end_id: 2, weight: 2 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 3, end_id: 1, weight: 3 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 3, end_id: 2, weight: 4 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 1, end_id: 4, weight: 1 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 2, end_id: 4, weight: 2 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 2, end_id: 5, weight: 9 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 4, end_id: 6, weight: 1 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 4, end_id: 7, weight: 2 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 6, end_id: 5, weight: 3 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 6, end_id: 8, weight: 4 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 7, end_id: 5, weight: 5 }));
        assert!(rules.vertices.contains(&Vertice { start_id: 7, end_id: 8, weight: 6 }));
        assert_eq!(rules.vertices.len(), 13);
    }
}