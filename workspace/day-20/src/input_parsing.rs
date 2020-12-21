use std::fs;
use crate::models::{Capture, Tile};

pub fn parse_text_file(file_name: String) -> Capture {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Capture {
    let mut capture : Capture = Default::default();

    content.split("\n\n")
        .for_each(|tile_string| {
            let data : Vec<&str> = tile_string.split_ascii_whitespace().collect();
            // Parse the id
            let id = data[1][..data[1].len() -1].parse::<u32>().unwrap();

            // Compute width/height
            let width = data[2].len();
            let height = data.len() - 2;

            // Parse the pixels
            let mut pixels : Vec<bool> = Vec::new();
            for y in 2..data.len() {
                for x in 0..data[y].len() {
                    if &data[y][x..x+1] == "#" {
                        pixels.push(true);
                    }
                    else {
                        pixels.push(false);
                    }
                }
            }

            // Insert the tile
            capture.insert(id, Tile {
                id,
                width,
                height,
                pixels
            });
        });

    capture
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        //region Example content
        let content = r#"Tile 2311:
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
"#.to_string();
        //endregion

        let capture = parse_string(content);

        assert_eq!(capture.len(), 9);
        assert_eq!(capture[&2311], Tile {
            id: 2311,
            width: 10,
            height: 10,
            pixels: vec![
                false, false, true, true, false, true, false, false, true, false,
                true, true, false, false, true, false, false, false, false, false,
                true, false, false, false, true, true, false, false, true, false,
                true, true, true, true, false, true, false, false, false, true,
                true, true, false, true, true, false, true, true, true, false,
                true, true, false, false, false, true, false, true, true, true,
                false, true, false, true, false, true, false, false, true, true,
                false, false, true, false, false, false, false, true, false, false,
                true, true, true, false, false, false, true, false, true, false,
                false, false, true, true, true, false, false, true, true, true,
            ]
        });

        assert_eq!(capture[&3079], Tile {
            id: 3079,
            width: 10,
            height: 10,
            pixels: vec![
                true, false, true, false, true, true, true, true, true, false, 
                false, true, false, false, true, true, true, true, true, true, 
                false, false, true, false, false, false, false, false, false, false, 
                true, true, true, true, true, true, false, false, false, false, 
                true, true, true, true, false, true, false, false, true, false, 
                false, true, false, false, false, true, false, true, true, false, 
                true, false, true, true, true, true, true, false, true, true, 
                false, false, true, false, true, true, true, false, false, false, 
                false, false, true, false, false, false, false, false, false, false, 
                false, false, true, false, true, true, true, false, false, false, 
            ]
        });
    }
}