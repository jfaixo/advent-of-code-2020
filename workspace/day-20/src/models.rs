use std::collections::HashMap;

pub type Capture = HashMap<u32, Tile>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tile {
    pub id: u32,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<bool>,
}

impl Tile {
    // Convention: borders will be considered in the "right" sense if they are read clockwise

    pub fn top_border(&self) -> Vec<bool> {
        self.pixels[0..self.width].to_vec()
    }

    pub fn bottom_border(&self) -> Vec<bool> {
        let offset = (self.height - 1) * self.width;
        let mut border = self.pixels[offset..offset + self.width].to_vec();
        border.reverse();
        border
    }

    pub fn left_border(&self) -> Vec<bool> {
        let mut border : Vec<bool> = Default::default();
        for y in 0..self.height {
            border.push(self.pixels[(self.height - 1 - y) * self.width]);
        }

        border
    }

    pub fn right_border(&self) -> Vec<bool> {
        let mut border : Vec<bool> = Default::default();
        for y in 0..self.height {
            border.push(self.pixels[y * self.width + self.width - 1]);
        }

        border
    }

    pub fn flip_vertically(&self) -> Tile {
        let mut new_tile = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                new_tile.pixels[y * self.width + x] = self.pixels[(self.height - 1 - y) * self.width + x];
            }
        }

        new_tile
    }

    pub fn flip_horizontally(&self) -> Tile {
        let mut new_tile = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                new_tile.pixels[y * self.width + x] = self.pixels[y * self.width + (self.width - 1 - x)];
            }
        }

        new_tile
    }

    pub fn rotate90(&self) -> Tile {
        let mut new_tile = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                new_tile.pixels[x * self.width + (self.width - 1 - y)] = self.pixels[y * self.width + x];
            }
        }

        new_tile
    }

    pub fn with_position(&self, position: &TilePosition) -> Tile {
        let mut new_tile = self.clone();
        if position.is_flipped_horizontally {
            new_tile = new_tile.flip_horizontally();
        }

        match position.rotation {
            TileRotation::Rot0 => {}
            TileRotation::Rot90 => {
                new_tile = new_tile.rotate90();
            }
            TileRotation::Rot180 => {
                new_tile = new_tile.rotate90();
                new_tile = new_tile.rotate90();
            }
            TileRotation::Rot270 => {
                new_tile = new_tile.rotate90();
                new_tile = new_tile.rotate90();
                new_tile = new_tile.rotate90();
            }
        }

        new_tile
    }

    pub fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.pixels[y * self.width + x] {
                    true => eprint!("#"),
                    false => eprint!("."),
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TileRotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TilePosition {
    pub is_flipped_horizontally: bool,
    pub rotation: TileRotation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Puzzle {
    pub width: usize,
    pub height: usize,
    pub pieces: Vec<(u32, TilePosition)>,
}
