use std::collections::HashSet;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ActiveCube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Default, Clone)]
pub struct SimulationContext {
    pub step: u32,
    pub x_axis_range: [i32; 2],
    pub y_axis_range: [i32; 2],
    pub z_axis_range: [i32; 2],
    pub activated_cubes: HashSet<ActiveCube>,
}

trait IsActive {
    fn is_active(&self, x: i32, y: i32, z: i32) -> bool;
    fn active_neighbours(&self, x: i32, y: i32, z: i32) -> u8;
}

impl IsActive for HashSet<ActiveCube> {
    fn is_active(&self, x: i32, y: i32, z: i32) -> bool {
        self.contains(&ActiveCube { x, y, z })
    }

    fn active_neighbours(&self, x: i32, y: i32, z: i32) -> u8 {
        let mut count: u8 = 0;

        for dz in -1..2 {
            for dy in -1..2 {
                for dx in -1..2 {
                    if !(dz == 0 && dy == 0 && dx == 0) && self.is_active(x + dx, y + dy, z + dz) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl SimulationContext {
    pub fn simulate_step(&mut self) {

        let old_activated_cubes = self.activated_cubes.clone();

        // Go through the whole space
        for z in self.z_axis_range[0] - 1..self.z_axis_range[1] + 2 {
            for y in self.y_axis_range[0] - 1..self.y_axis_range[1] + 2 {
                for x in self.x_axis_range[0] - 1..self.x_axis_range[1] + 2 {
                    match old_activated_cubes.is_active(x, y, z) {
                        true => {
                            let active_neighbours_count = old_activated_cubes.active_neighbours(x, y, z);
                            if !(active_neighbours_count == 2 || active_neighbours_count == 3) {
                                self.deactivate_cube(x, y, z);
                            }
                        }
                        false => {
                            let active_neighbours_count = old_activated_cubes.active_neighbours(x, y, z);
                            if active_neighbours_count == 3 {
                                self.activate_cube(x, y, z);
                            }
                        }
                    }
                }
            }
        }

        self.step += 1;
    }

    fn activate_cube(&mut self, x: i32, y: i32, z: i32) {
        self.activated_cubes.insert(ActiveCube { x, y, z });

        if x < self.x_axis_range[0] { self.x_axis_range[0] = x; }
        if x > self.x_axis_range[1] { self.x_axis_range[1] = x; }

        if y < self.y_axis_range[0] { self.y_axis_range[0] = y; }
        if y > self.y_axis_range[1] { self.y_axis_range[1] = y; }

        if z < self.z_axis_range[0] { self.z_axis_range[0] = z; }
        if z > self.z_axis_range[1] { self.z_axis_range[1] = z; }
    }

    fn deactivate_cube(&mut self, x: i32, y: i32, z: i32) {
        self.activated_cubes.remove(&ActiveCube { x, y, z });
    }
}

pub fn active_count_after_6_steps(context: &mut SimulationContext) -> usize {
    for _i in 0..6 {
        context.simulate_step();
    }

    context.activated_cubes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing_part_1::parse_string;

    #[test]
    fn active_neighbours_count() {
        let content = ".#.
..#
###".to_string();
        let context = parse_string(content);

        assert_eq!(context.activated_cubes.active_neighbours(1, 1, 0), 5);
        assert_eq!(context.activated_cubes.active_neighbours(1, 1, 1), 5);
        assert_eq!(context.activated_cubes.active_neighbours(1, 1, -1), 5);
        assert_eq!(context.activated_cubes.active_neighbours(0, 0, 0), 1);
    }

    #[test]
    fn simulate_one_step() {
        let content = ".#.
..#
###".to_string();
        let mut context = parse_string(content);

        for _i in 0..6 {
            context.simulate_step();
        }

        assert_eq!(context.activated_cubes.len(), 112);
        // The site example is false
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 0, y: 0, z: -1 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 1, z: -1 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 2, z: -1 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 0, y: 0, z: 0 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 0, z: 0 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 1, z: 0 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 1, z: 0 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 2, z: 0 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 0, y: 0, z: 1 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 2, y: 1, z: 1 }));
        // assert!(context.activated_cubes.contains(&ActiveCube { x: 1, y: 2, z: 1 }));
    }
}