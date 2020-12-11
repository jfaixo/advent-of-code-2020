use crate::sit_layout::{SitLayout, LayoutPosition};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Default)]
pub struct SimulationState {
    pub occupied_seats : HashSet<usize>
}

#[derive(Debug, Default)]
pub struct SeatAdjacencyMap {
    pub adjacency_map : HashMap<usize, Vec<usize>>
}

fn adjacent_occupied_sits(adjacency_map: &HashMap<usize, Vec<usize>>, current_state: &SimulationState, index: usize) -> u32 {
    let mut count = 0;

    for i in &adjacency_map[&index] {
        if current_state.occupied_seats.contains(&i) {
            count += 1;
        }
    }

    count
}

pub fn seating_simulation(sit_layout: &SitLayout, seat_adjacency_map: &SeatAdjacencyMap, crowd_tolerancy: u32) -> usize {

    let mut current_state : SimulationState = Default::default();

    // Simulation loop
    loop {
        let mut new_state: SimulationState = Default::default();

        // For each seat
        for y in 0..sit_layout.height {
            for x in 0..sit_layout.width {
                if sit_layout.layout[y * sit_layout.width + x] == LayoutPosition::Seat {
                    // Apply the rules
                    let current_position_index = y * sit_layout.width + x;

                    // #1 : if a seat is empty and no occupied adjacent seats => sit
                    if !current_state.occupied_seats.contains(&current_position_index) &&
                        adjacent_occupied_sits(&seat_adjacency_map.adjacency_map, &current_state, current_position_index) == 0 {
                        new_state.occupied_seats.insert(current_position_index);
                    }

                    // #2 : if a seat is occupied and 4 or more adjacent seat occupied => leave
                    // We can invert this statement to know when to remain sitted
                    // If a seat is occupied, and adjacent seats occupied < 3, stay sit
                    if current_state.occupied_seats.contains(&current_position_index) &&
                        adjacent_occupied_sits(&seat_adjacency_map.adjacency_map, &current_state, current_position_index) < crowd_tolerancy {
                        new_state.occupied_seats.insert(current_position_index);
                    }
                }
            }
        }

        // Test if we reached a stable layout
        if new_state.occupied_seats == current_state.occupied_seats {
            break;
        }
        // Step the simulation
        current_state = new_state;
    }

    current_state.occupied_seats.len()
}
