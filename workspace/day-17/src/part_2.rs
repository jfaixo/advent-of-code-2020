use std::collections::HashSet;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ActiveCube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Debug, Default, Clone)]
pub struct SimulationContext {
    pub step: u32,
    pub x_axis_range: [i32; 2],
    pub y_axis_range: [i32; 2],
    pub z_axis_range: [i32; 2],
    pub w_axis_range: [i32; 2],
    pub activated_cubes: HashSet<ActiveCube>,
}


trait IsActive {
    fn is_active(&self, x: i32, y: i32, z: i32, w: i32) -> bool;
    fn active_neighbours(&self, x: i32, y: i32, z: i32, w: i32) -> u8;
}

impl IsActive for HashSet<ActiveCube> {
    fn is_active(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        self.contains(&ActiveCube { x, y, z, w })
    }

    fn active_neighbours(&self, x: i32, y: i32, z: i32, w: i32) -> u8 {
        let mut count: u8 = 0;

        for dw in -1..2 {
            for dz in -1..2 {
                for dy in -1..2 {
                    for dx in -1..2 {
                        if !(dz == 0 && dy == 0 && dx == 0 && dw == 0) && self.is_active(x + dx, y + dy, z + dz, w + dw) {
                            count += 1;
                        }
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
        for w in self.w_axis_range[0] - 1..self.w_axis_range[1] + 2 {
            for z in self.z_axis_range[0] - 1..self.z_axis_range[1] + 2 {
                for y in self.y_axis_range[0] - 1..self.y_axis_range[1] + 2 {
                    for x in self.x_axis_range[0] - 1..self.x_axis_range[1] + 2 {
                        match old_activated_cubes.is_active(x, y, z, w) {
                            true => {
                                let active_neighbours_count = old_activated_cubes.active_neighbours(x, y, z, w);
                                if !(active_neighbours_count == 2 || active_neighbours_count == 3) {
                                    self.deactivate_cube(x, y, z, w);
                                }
                            }
                            false => {
                                let active_neighbours_count = old_activated_cubes.active_neighbours(x, y, z, w);
                                if active_neighbours_count == 3 {
                                    self.activate_cube(x, y, z, w);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.step += 1;
    }

    fn activate_cube(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.activated_cubes.insert(ActiveCube { x, y, z, w });

        if x < self.x_axis_range[0] { self.x_axis_range[0] = x; }
        if x > self.x_axis_range[1] { self.x_axis_range[1] = x; }

        if y < self.y_axis_range[0] { self.y_axis_range[0] = y; }
        if y > self.y_axis_range[1] { self.y_axis_range[1] = y; }

        if z < self.z_axis_range[0] { self.z_axis_range[0] = z; }
        if z > self.z_axis_range[1] { self.z_axis_range[1] = z; }

        if w < self.w_axis_range[0] { self.w_axis_range[0] = w; }
        if w > self.w_axis_range[1] { self.w_axis_range[1] = w; }
    }

    fn deactivate_cube(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.activated_cubes.remove(&ActiveCube { x, y, z, w });
    }
}

pub fn active_count_after_6_steps(context: &mut SimulationContext) -> usize {
    for _i in 0..6 {
        context.simulate_step();
    }

    context.activated_cubes.len()
}