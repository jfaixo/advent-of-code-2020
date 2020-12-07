use crate::regulation_rules::RegulationRules;
use std::collections::HashSet;

// Let's take the hypothesis that this is an acyclic graph.
// Implement a recursive postorder traversal
pub fn containing_bags_count(rules: &RegulationRules, bag_id: u32) -> u32 {
    let mut containing_bags : HashSet<u32> = HashSet::new();
    bag_count(rules, bag_id, &mut containing_bags);

    containing_bags.len() as u32
}

fn bag_count(rules: &RegulationRules, bag_id: u32, containing_bags: &mut HashSet<u32>) -> u32 {
    let parent_counts : u32 = rules.vertices.iter().map(|vertice| {
        if vertice.end_id == bag_id && !containing_bags.contains(&vertice.start_id) {
            containing_bags.insert(vertice.start_id);
            bag_count(rules, vertice.start_id, containing_bags)
        } else {
            0
        }
    }).sum();

    1 + parent_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_1_full() {
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

        let can_contain = containing_bags_count(&rules, rules.nodes["shiny gold"]);

        assert_eq!(can_contain, 4);
    }
}