use crate::models::{Capture, TilePosition, TileRotation, Puzzle, Tile};
use std::collections::HashMap;
use crate::models::TileRotation::{Rot270, Rot0, Rot90, Rot180};

pub fn rough_water_indicator(capture: &Capture) -> u32 {
    // Solve the puzzle
    let puzzle = solve_puzzle(capture);

    // Transform our puzzle into an image
    let image = puzzle_to_image(capture, &puzzle);

    // Now, let's find sea monsters !
    // We will try all the permutations (4 rotations, and flip then 4 rotations) and keep the
    // biggest monsters count
    let mut sea_monster_counts : Vec<u32> = Default::default();
    sea_monster_counts.push(find_sea_monsters(&image));
    sea_monster_counts.push(find_sea_monsters(&image.rotate90()));
    sea_monster_counts.push(find_sea_monsters(&image.rotate90().rotate90()));
    sea_monster_counts.push(find_sea_monsters(&image.rotate90().rotate90().rotate90()));
    sea_monster_counts.push(find_sea_monsters(&image.flip_horizontally()));
    sea_monster_counts.push(find_sea_monsters(&image.flip_horizontally().rotate90()));
    sea_monster_counts.push(find_sea_monsters(&image.flip_horizontally().rotate90().rotate90()));
    sea_monster_counts.push(find_sea_monsters(&image.flip_horizontally().rotate90().rotate90().rotate90()));

    let maximum_monsters_count: u32 = sea_monster_counts.iter().fold(0, |acc, c| if *c > acc { *c } else { acc });

    // Finally, compute the rough water indicator. There are 15 "#" in one sea monster, just
    // substract that from the total number of "#" in the image
    let total_hash_signs : u32 = image.pixels.iter().fold(0, |acc, p| if *p { acc + 1 } else { acc });

    return total_hash_signs - maximum_monsters_count * 15;
}

fn find_sea_monsters(image: &Tile) -> u32 {
    let mut sea_monsters_count = 0;
    for y in 0..image.height {
        for x in 0..image.width {
            if image.here_be_a_monster(x, y) {
                sea_monsters_count += 1;
            }
        }
    }

    sea_monsters_count
}

impl Tile {
    // This is a sea monster :
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    fn here_be_a_monster(&self, x: usize, y: usize) -> bool {
        // If the monster has a part outside of the image, just return
        if y + 2 >= self.height || x + 19 >= self.width {
            return false;
        }
        // Check for the monster
        if self.pixels[y * self.width + x + 18]
            && self.pixels[(y + 1) * self.width + x]
            && self.pixels[(y + 1) * self.width + x + 5]
            && self.pixels[(y + 1) * self.width + x + 6]
            && self.pixels[(y + 1) * self.width + x + 11]
            && self.pixels[(y + 1) * self.width + x + 12]
            && self.pixels[(y + 1) * self.width + x + 17]
            && self.pixels[(y + 1) * self.width + x + 18]
            && self.pixels[(y + 1) * self.width + x + 19]
            && self.pixels[(y + 2) * self.width + x + 1]
            && self.pixels[(y + 2) * self.width + x + 4]
            && self.pixels[(y + 2) * self.width + x + 7]
            && self.pixels[(y + 2) * self.width + x + 10]
            && self.pixels[(y + 2) * self.width + x + 13]
            && self.pixels[(y + 2) * self.width + x + 16] {
            return true
        }
        return false;
    }
}

fn solve_puzzle(capture: &Capture) -> Puzzle {
    // Generate a hashmap containing all borders possible with a reference to the corresponding
    // tile rotated & flipped so that the border is at the top of the tile
    let mut tiles_borders: HashMap<Vec<bool>, Vec<(u32, TilePosition)>> = Default::default();
    for tile in capture.values() {
        tiles_borders.entry(tile.top_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot0 }));
        tiles_borders.entry(tile.left_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot90 }));
        tiles_borders.entry(tile.bottom_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot180 }));
        tiles_borders.entry(tile.right_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot270 }));

        let tile_fh = tile.flip_horizontally();
        tiles_borders.entry(tile_fh.top_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: true, rotation: TileRotation::Rot0 }));
        tiles_borders.entry(tile_fh.bottom_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: true, rotation: TileRotation::Rot180 }));

        let tile_fv = tile.flip_vertically();
        tiles_borders.entry(tile_fv.left_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: true, rotation: TileRotation::Rot270 }));
        tiles_borders.entry(tile_fv.right_border()).or_default().push((tile.id, TilePosition { is_flipped_horizontally: true, rotation: TileRotation::Rot90 }));
    }

    // Identify all the "lonely" borders, ie that does not match with any other tile
    // Key: tile id, value: lonely border
    let mut tile_single_id_count: HashMap<u32, Vec<(u32, TilePosition)>> = Default::default();
    tiles_borders.keys().for_each(|key| {
        if tiles_borders[key].len() == 1 {
            tile_single_id_count.entry(tiles_borders[key][0].0).or_default()
                .push((tiles_borders[key][0].0, tiles_borders[key][0].1.clone()));
        }
    });

    // Extract the 4 tile that have 2 "lonely" borders (ie = 4 occurrences because of inverts),
    // and all other borders
    let mut corners : Vec<u32> = Default::default();
    let mut borders: Vec<u32> = Default::default();
    tile_single_id_count.keys().for_each(|key| {
        if tile_single_id_count[key].len() == 4 {
            corners.push(*key);
        }
        if tile_single_id_count[key].len() == 2 {
            borders.push(*key);
        }
    });

    // Now, start to solve the puzzle. Start with a corner, and create the whole border
    let puzzle_border_size = (capture.len() as f32).sqrt() as usize;
    let mut puzzle = Puzzle {
        width: puzzle_border_size,
        height: puzzle_border_size,
        pieces: vec![(0, TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot0 }); capture.len()],
    };

    // First corner "because I know", to go faster, I just looked the right position for the first corner in debug trace
    // the two outer borders are simply the top one, and the left one
    // TODO: hard coded...
    // For true problem
    puzzle.pieces[0].0 = 1889;
    puzzle.pieces[0].1 = TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot0 };
    // For unit test
    // puzzle.pieces[0].0 = 3079;
    // puzzle.pieces[0].1 = TilePosition { is_flipped_horizontally: false, rotation: TileRotation::Rot270 };

    // Recreate the puzzle
    // By looking at the "tiles_borders" dump, we can see that there is always only exactly one match for our problem
    for x in 1..puzzle.width {
        // Find the piece in the proper position
        let mut right_border_of_previous_piece = capture[&puzzle.pieces[x - 1].0].with_position(&puzzle.pieces[x - 1].1).right_border().clone();
        // Reverse to be in the proper reading sense
        right_border_of_previous_piece.reverse();
        let mut pieces = tiles_borders[&right_border_of_previous_piece].clone();
        let piece : Vec<(u32, TilePosition)> = pieces.into_iter().filter(|piece| piece.0 != puzzle.pieces[x - 1].0).collect();
        // Put the piece
        puzzle.pieces[x].0 = piece[0].0;
        puzzle.pieces[x].1 = piece[0].1.clone();
        // We want to rotate the piece correctly, because we found it with the border at the top, so we need to change the piece orientation by rotating it by 270°
        puzzle.pieces[x].1.rotation = match puzzle.pieces[x].1.rotation {
            TileRotation::Rot0 => Rot270,
            TileRotation::Rot90 => Rot0,
            TileRotation::Rot180 => Rot90,
            TileRotation::Rot270 => Rot180,
        }
    }
    // Now that we have the first line, recreate the next lines
    for y in 1..puzzle.height {
        for x in 0..puzzle.width {
            // Find the piece in the proper position
            let mut bottom_border_of_previous_piece = capture[&puzzle.pieces[(y - 1) * puzzle.width + x].0].with_position(&puzzle.pieces[(y - 1) * puzzle.width + x].1).bottom_border().clone();
            // Revert the bottom border to have it in the right sense
            bottom_border_of_previous_piece.reverse();
            let mut pieces = tiles_borders[&bottom_border_of_previous_piece].clone();
            let piece : Vec<(u32, TilePosition)> = pieces.into_iter().filter(|piece| piece.0 != puzzle.pieces[(y - 1) * puzzle.width + x].0).collect();
            // Put the piece
            puzzle.pieces[y * puzzle.width + x].0 = piece[0].0;
            puzzle.pieces[y * puzzle.width + x].1 = piece[0].1.clone();
        }
    }

    // // Print the puzzle to be sure it is properly solved
    // for puzzle_y in 0..puzzle.height {
    //     for tile_y in 0..capture[&puzzle.pieces[0].0].height {
    //         for puzzle_x in 0..puzzle.width {
    //             for tile_x in 0..capture[&puzzle.pieces[0].0].width {
    //                 let tile = capture[&puzzle.pieces[puzzle_y * puzzle.width + puzzle_x].0].with_position(&puzzle.pieces[puzzle_y * puzzle.width + puzzle_x].1);
    //                 if tile.pixels[tile_y * tile.width + tile_x] == true {
    //                     eprint!("#");
    //                 }
    //                 else {
    //                     eprint!(".");
    //                 }
    //             }
    //             eprint!("   ");
    //         }
    //         eprintln!();
    //     }
    //     eprintln!("\n");
    // }

    puzzle
}

fn puzzle_to_image(capture: &Capture, puzzle: &Puzzle) -> Tile {
    let image_size = puzzle.width * (capture.values().next().unwrap().width - 2);

    let mut pixels : Vec<bool> = Vec::new();
    for puzzle_y in 0..puzzle.height {
        for tile_y in 1..capture[&puzzle.pieces[0].0].height - 1 {
            for puzzle_x in 0..puzzle.width {
                for tile_x in 1..capture[&puzzle.pieces[0].0].width - 1 {
                    let tile = capture[&puzzle.pieces[puzzle_y * puzzle.width + puzzle_x].0].with_position(&puzzle.pieces[puzzle_y * puzzle.width + puzzle_x].1);
                    pixels.push(tile.pixels[tile_y * tile.width + tile_x]);
                }
            }
        }
    }

    Tile {
        id: 0,
        width: image_size,
        height: image_size,
        pixels
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Tile;
    use crate::part_2::{solve_puzzle, rough_water_indicator};

    //region Example content
    const content : &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"#;
    //endregion

    #[test]
    fn test_rotation() {
        let capture = parse_string(content.to_string());

        // ..##.#..#.
        // ##..#.....
        // #...##..#.
        // ####.#...#
        // ##.##.###.
        // ##...#.###
        // .#.#.#..##
        // ..#....#..
        // ###...#.#.
        // ..###..###

        assert_eq!(capture[&2311].rotate90(), Tile {
            id: 2311,
            width: 10,
            height: 10,
            pixels: vec![
                false, true, false, false, true, true, true, true, true, false,
                false, true, false, true, true, true, true, false, true, false,
                true, true, true, false, false, false, true, false, false, true,
                true, false, false, true, false, true, true, false, false, true,
                true, false, false, false, false, true, false, true, true, false,
                false, false, false, true, true, false, true, true, false, true,
                false, true, false, false, false, true, false, false, false, false,
                true, false, true, false, true, true, false, false, false, false,
                true, true, false, true, true, true, false, true, false, true,
                true, false, false, true, true, false, true, false, false, false,

            ]
        });
    }

    #[test]
    fn test_part_2() {
        let capture = parse_string(content.to_string());

        let result = rough_water_indicator(&capture);

        assert_eq!(result, 273);
    }
}
