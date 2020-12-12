use crate::navigation_instructions::{NavigationInstructions, NavigationAction};

#[derive(Debug, Eq, PartialEq)]
struct NavigationState {
    x: i32,
    y: i32,
    orientation: i32,       // 0 is poiting right, as in a trigonometric circle
}

pub fn manhattan_distance_part_1(instructions: &NavigationInstructions) -> i32 {
    let state = simulate_navigation(instructions);
    state.x.abs() + state.y.abs()
}

fn simulate_navigation(instructions: &NavigationInstructions) -> NavigationState {

    let mut current_position = NavigationState { x: 0, y: 0, orientation: 0 };

    for instruction in instructions {
        match instruction.action {
            NavigationAction::North => current_position.y += instruction.value,
            NavigationAction::South => current_position.y -= instruction.value,
            NavigationAction::East => current_position.x += instruction.value,
            NavigationAction::West => current_position.x -= instruction.value,
            NavigationAction::Left => current_position.orientation = (current_position.orientation + instruction.value + 360) % 360,
            NavigationAction::Right => current_position.orientation = (current_position.orientation - instruction.value + 360) % 360,
            NavigationAction::Forward => {
                match current_position.orientation {
                    0 => current_position.x += instruction.value,
                    90 => current_position.y += instruction.value,
                    180 => current_position.x -= instruction.value,
                    270 => current_position.y -= instruction.value,
                    _ => panic!("Should not happen: {:?}, {:?}", instruction.action, current_position.orientation)
                }
            }
        }
    }

    current_position
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_1::manhattan_distance_part_1;

    #[test]
    fn test_example() {
        let content = "F10
N3
F7
R90
F11
".to_string();
        let instructions = parse_string(content);
        let distance = manhattan_distance_part_1(&instructions);
        assert_eq!(distance, 25);
    }
}