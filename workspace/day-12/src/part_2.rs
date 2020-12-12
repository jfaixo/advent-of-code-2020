use crate::navigation_instructions::{NavigationInstructions, NavigationAction};

#[derive(Debug, Eq, PartialEq)]
struct NavigationState {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl NavigationState {
    fn rotate_waypoint(&mut self, angle: i32) {
        // Compute the rotation to apply in [0, 360[
        let rotate_90_count = (angle + 360) % 360 / 90;

        for _i in 0..rotate_90_count {
            let new_waypoint_x = - self.waypoint_y;
            let new_waypoint_y = self.waypoint_x;
            self.waypoint_x = new_waypoint_x;
            self.waypoint_y = new_waypoint_y;
        }
    }
}

pub fn manhattan_distance_part_2(instructions: &NavigationInstructions) -> i32 {
    let state = simulate_navigation(instructions);
    state.x.abs() + state.y.abs()
}

fn simulate_navigation(instructions: &NavigationInstructions) -> NavigationState {

    let mut current_position = NavigationState { x: 0, y: 0, waypoint_x: 10, waypoint_y: 1 };

    for instruction in instructions {
        match instruction.action {
            NavigationAction::North => current_position.waypoint_y += instruction.value,
            NavigationAction::South => current_position.waypoint_y -= instruction.value,
            NavigationAction::East => current_position.waypoint_x += instruction.value,
            NavigationAction::West => current_position.waypoint_x -= instruction.value,
            NavigationAction::Left => current_position.rotate_waypoint(instruction.value),
            NavigationAction::Right => current_position.rotate_waypoint(-instruction.value),
            NavigationAction::Forward => {
                current_position.x += current_position.waypoint_x * instruction.value;
                current_position.y += current_position.waypoint_y * instruction.value;
            }
        }
    }

    current_position
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_2::manhattan_distance_part_2;

    #[test]
    fn test_example() {
        let content = "F10
N3
F7
R90
F11
".to_string();
        let instructions = parse_string(content);
        let distance = manhattan_distance_part_2(&instructions);
        assert_eq!(distance, 286);
    }
}