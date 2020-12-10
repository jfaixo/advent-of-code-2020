use crate::adapter_input::AdapterInput;
use std::collections::HashMap;

// One hundred lines in the input data, we can safely try to solve that recursively (a depth of 100 won't stack overflow)
pub fn count_valid_adapters_chain_possibilities(input: &AdapterInput) -> u64 {
    let mut serie = input.adapters.clone();
    // Insert the root node, which is our start point
    serie.insert(0, 0);
    serie.push(input.device_joltage);

    // Build an oriented graph of the possibilities
    // Vertices that target the next possible adapters
    let mut oriented_vertices : Vec<Vec<usize>> = Vec::new();
    for current_node in 0..serie.len() {
        let mut children: Vec<usize> = Vec::new();

        let mut i : usize = 1;
        while current_node + i < serie.len() && serie[current_node + i] <= serie[current_node] + 3 {
            children.push(current_node + i);
            i += 1;
        }
        oriented_vertices.push(children);
    }

    // This hash map will store the possibilities count for each node, if already computed
    let mut nodes_possibilities: HashMap<usize, u64> = HashMap::new();
    // Graph traversal that computes the possibilities below each child
    count_for_node(&oriented_vertices, 0, &mut nodes_possibilities);
    // The number of possibilities of the root node is our solution
    let zero : usize = 0;
    nodes_possibilities[&zero]
}

fn count_for_node(oriented_vertices: &Vec<Vec<usize>>, current_node: usize, nodes_possibilities: &mut HashMap<usize, u64>) {
    // Lazy: only compute count for node that have not already been computed
    if nodes_possibilities.contains_key(&current_node) == false {
        // If the node has no child (ie the end node), insert 1
        if oriented_vertices[current_node].len() == 0 {
            nodes_possibilities.insert(current_node, 1);
        }
        else {
            // Compute the sum of possibilities of all childs
            let mut aggregated_possibilities : u64 = 0;
            for child in &oriented_vertices[current_node] {
                count_for_node(oriented_vertices, *child, nodes_possibilities);
                aggregated_possibilities += nodes_possibilities[&child];
            }

            nodes_possibilities.insert(current_node, aggregated_possibilities);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_2_example_1() {
        let content = "16
10
15
5
1
11
7
19
6
12
4
".to_string();
        let input = parse_string(content);
        let result = count_valid_adapters_chain_possibilities(&input);
        assert_eq!(result, 8);
    }


    #[test]
    fn test_part_2_example_2() {
        let content = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
".to_string();
        let input = parse_string(content);
        let result = count_valid_adapters_chain_possibilities(&input);
        assert_eq!(result, 19208);
    }
}