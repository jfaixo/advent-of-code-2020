use crate::sit_layout::{SitLayout, LayoutPosition};
use std::collections::HashMap;
use crate::seat_simulation::SeatAdjacencyMap;

pub fn compute_adjacency_map_part_2(sit_layout: &SitLayout) -> SeatAdjacencyMap {
    let mut adjacency_map : HashMap<usize, Vec<usize>> = HashMap::with_capacity(sit_layout.width * sit_layout.height);

    // For each position
    for y0 in 0..sit_layout.height {
        for x0 in 0..sit_layout.width {
            let index0 = y0 * sit_layout.width + x0;
            // If this is a seat
            if sit_layout.layout[index0] == LayoutPosition::Seat {

                // Find the adjacent seat index by exploring each direction
                let mut adjacent_seat_index_list : Vec<usize> = Vec::new();

                // Top
                let mut x = x0 as i32;
                let mut y = y0 as i32 - 1;
                while y >= 0 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    y -= 1;
                }

                // Bottom
                x = x0 as i32;
                y = y0 as i32 + 1;
                while y < sit_layout.height as i32 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    y += 1;
                }

                // Left
                x = x0 as i32 - 1;
                y = y0 as i32;
                while x >= 0 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x -= 1;
                }

                // Right
                x = x0 as i32 + 1;
                y = y0 as i32;
                while x < sit_layout.width as i32 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x += 1;
                }

                // Top left
                x = x0 as i32 - 1;
                y = y0 as i32 - 1;
                while x >= 0 && y >= 0 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x -= 1;
                    y -= 1;
                }

                // Top Right
                x = x0 as i32 + 1;
                y = y0 as i32 - 1;
                while x < sit_layout.width as i32 && y >= 0 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x += 1;
                    y -= 1;
                }

                // Bottom Right
                x = x0 as i32 + 1;
                y = y0 as i32 + 1;
                while x < sit_layout.width as i32 && y < sit_layout.height as i32 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x += 1;
                    y += 1;
                }

                // Bottom Left
                x = x0 as i32 - 1;
                y = y0 as i32 + 1;
                while x >= 0 as i32 && y < sit_layout.height as i32 {
                    if sit_layout.layout[(y * sit_layout.width as i32 + x as i32) as usize] == LayoutPosition::Seat {
                        adjacent_seat_index_list.push((y * sit_layout.width as i32 + x) as usize);
                        break;
                    }
                    x -= 1;
                    y += 1;
                }

                adjacency_map.insert(index0, adjacent_seat_index_list);
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
        let seat_adjacency_map = compute_adjacency_map_part_2(&input);
        let result = seating_simulation(&input, &seat_adjacency_map, 5);
        assert_eq!(result, 26);
    }
}