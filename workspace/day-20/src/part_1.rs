use crate::models::{Capture};
use std::collections::HashMap;

pub fn multiplication_of_corners(capture: &Capture) -> u64 {
    let mut borders : HashMap<Vec<bool>, Vec<u32>> = Default::default();

    // Generate a hashmap containing all borders possible with a reference to the corresponding tile
    for tile in capture.values() {
        borders.entry(tile.top_border()).or_default().push(tile.id);
        borders.entry(tile.bottom_border()).or_default().push(tile.id);
        borders.entry(tile.left_border()).or_default().push(tile.id);
        borders.entry(tile.right_border()).or_default().push(tile.id);

        let tile_fh = tile.flip_horizontally();
        borders.entry(tile_fh.top_border()).or_default().push(tile.id);
        borders.entry(tile_fh.bottom_border()).or_default().push(tile.id);

        let tile_fv = tile.flip_vertically();
        borders.entry(tile_fv.left_border()).or_default().push(tile.id);
        borders.entry(tile_fv.right_border()).or_default().push(tile.id);
    }

    // Identify all the "lonely" borders, ie that does not match with any other tile
    // Key: tile id, value: number of lonely borders
    let mut tile_single_id_count: HashMap<u32, u32> = Default::default();
    borders.keys().for_each(|key| {
        if borders[key].len() == 1 {
            let count = tile_single_id_count.entry(borders[key][0]).or_default();
            *count += 1;
        }
    });

    // Extract the 4 tile that have 2 "lonely" borders (ie = 4 occurrences because of inverts)
    let mut corners : Vec<u32> = Default::default();
    tile_single_id_count.keys().for_each(|key| {
        if tile_single_id_count[key] == 4 {
            corners.push(*key);
        }
    });

    corners.iter().fold(1, |acc, x| acc * (*x as u64))
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Tile;
    use crate::part_1::multiplication_of_corners;

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
    fn test_borders_and_invert() {
        //region Example content
        //endregion

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

        assert_eq!(capture[&2311].top_border(), vec![false, false, true, true, false, true, false, false, true, false]);
        assert_eq!(capture[&2311].bottom_border(), vec![true, true, true, false, false, true, true, true, false, false]);
        assert_eq!(capture[&2311].left_border(), vec![false, true, false, false, true, true, true, true, true, false]);
        assert_eq!(capture[&2311].right_border(), vec![false, false, false, true, false, true, true, false, false, true]);

        assert_eq!(capture[&2311].flip_vertically(), Tile {
            id: 2311,
            width: 10,
            height: 10,
            pixels: vec![
                false, false, true, true, true, false, false, true, true, true,
                true, true, true, false, false, false, true, false, true, false,
                false, false, true, false, false, false, false, true, false, false,
                false, true, false, true, false, true, false, false, true, true,
                true, true, false, false, false, true, false, true, true, true,
                true, true, false, true, true, false, true, true, true, false,
                true, true, true, true, false, true, false, false, false, true,
                true, false, false, false, true, true, false, false, true, false,
                true, true, false, false, true, false, false, false, false, false,
                false, false, true, true, false, true, false, false, true, false,
            ]
        });

        assert_eq!(capture[&2311].flip_horizontally(), Tile {
            id: 2311,
            width: 10,
            height: 10,
            pixels: vec![
                 false, true, false, false, true, false, true, true, false,false,
                 false, false, false, false, false, true, false, false, true,true,
                 false, true, false, false, true, true, false, false, false,true,
                 true, false, false, false, true, false, true, true, true,true,
                 false, true, true, true, false, true, true, false, true,true,
                 true, true, true, false, true, false, false, false, true,true,
                 true, true, false, false, true, false, true, false, true,false,
                 false, false, true, false, false, false, false, true, false,false,
                 false, true, false, true, false, false, false, true, true,true,
                 true, true, true, false, false, true, true, true, false,false,
            ]
        });
    }

    #[test]
    fn test_part_1() {
        let capture = parse_string(content.to_string());

        let result = multiplication_of_corners(&capture);

        assert_eq!(result, 20899048083289);
    }
}
