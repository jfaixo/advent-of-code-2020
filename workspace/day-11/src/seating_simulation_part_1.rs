use crate::sit_layout::{SitLayout, LayoutPosition};
use std::collections::HashMap;
use crate::seat_simulation::SeatAdjacencyMap;

pub fn compute_adjacency_map_part_1(sit_layout: &SitLayout) -> SeatAdjacencyMap {
    let mut adjacency_map : HashMap<usize, Vec<usize>> = HashMap::with_capacity(sit_layout.width * sit_layout.height);

    // For each position
    for y in 0..sit_layout.height {
        for x in 0..sit_layout.width {
            let index = y * sit_layout.width + x;
            // If this is a seat
            if sit_layout.layout[index] == LayoutPosition::Seat {
                // Find the adjacent seat index
                let mut adjacent_seat_index_list : Vec<usize> = Vec::new();
                for y_delta in -1..2 as i32 {
                    for x_delta in -1..2 as i32 {
                        let x_pos = x as i32 + x_delta;
                        let y_pos = y as i32 + y_delta;

                        // Do not try to count outside the layout, or the person at the evaluated coordinates
                        if x_pos < 0 || x_pos == sit_layout.width as i32 ||
                            y_pos < 0 || y_pos == sit_layout.height as i32 ||
                            (x_pos == x as i32 && y_pos == y as i32) {
                            continue;
                        }
                        adjacent_seat_index_list.push((y_pos * sit_layout.width as i32 + x_pos) as usize);
                    }
                }
                adjacency_map.insert(index, adjacent_seat_index_list);
            }
        }
    }

    SeatAdjacencyMap {
        adjacency_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;
    use crate::seat_simulation::seating_simulation;

    #[test]
    fn test_part_1_example() {
        let content = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
".to_string();
        let input = parse_string(content);
        let seat_adjacency_map = compute_adjacency_map_part_1(&input);
        let result = seating_simulation(&input, &seat_adjacency_map, 4);
        assert_eq!(result, 37);
    }
}