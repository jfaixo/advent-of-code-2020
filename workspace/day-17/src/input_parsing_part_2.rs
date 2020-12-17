use std::fs;
use crate::part_2::{SimulationContext, ActiveCube};

pub fn parse_text_file(file_name: String) -> SimulationContext {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> SimulationContext {
    let mut context: SimulationContext = Default::default();

    let lines : Vec<&str> = content.split("\n").collect();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if &lines[y][x..x+1] == "#" {
                context.activated_cubes.insert(ActiveCube {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0
                });
            }
        }
    }
    context.x_axis_range[1] = lines[0].len() as i32 - 1;
    context.y_axis_range[1] = lines.len() as i32 - 1;

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = ".#.
..#
###".to_string();
        let context = parse_string(content);

        assert_eq!(context.x_axis_range, [0, 2]);
        assert_eq!(context.y_axis_range, [0, 2]);
        assert_eq!(context.z_axis_range, [0, 0]);

        assert_eq!(context.activated_cubes.len(), 5);
        assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 0, z: 0, w: 0 }));
        assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 1, z: 0, w: 0 }));
        assert!(context.activated_cubes.contains(&ActiveCube { x: 0, y: 2, z: 0, w: 0 }));
        assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 2, z: 0, w: 0 }));
        assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 2, z: 0, w: 0 }));
    }
}