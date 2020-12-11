use std::fs;
use crate::sit_layout::{SitLayout, LayoutPosition};

pub fn parse_text_file(file_name: String) -> SitLayout {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> SitLayout {
    let rows : Vec<&str> = content.split_ascii_whitespace().collect();
    let width : usize = rows[0].len();
    let height : usize = rows.len();
    let mut layout : Vec<LayoutPosition> = Vec::with_capacity(width * height);

    for row in rows {
        row.chars().for_each(|c| {
            match c {
                'L' => layout.push(LayoutPosition::Seat),
                '.' => layout.push(LayoutPosition::Floor),
                _ => {}
            }
        });
    }

    SitLayout {
        width,
        height,
        layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sit_layout::LayoutPosition::{Seat, Floor};

    #[test]
    fn parse_example_case() {
        let content = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
".to_string();
        let layout = parse_string(content);

        assert_eq!(layout.width, 10);
        assert_eq!(layout.height, 10);
        assert_eq!(layout.layout, vec![
            Seat, Floor, Seat, Seat, Floor, Seat, Seat, Floor, Seat, Seat,      // 1
            Seat,Seat,Seat,Seat,Seat,Seat,Seat,Floor,Seat,Seat,                 // 2
            Seat,Floor,Seat,Floor,Seat,Floor,Floor,Seat,Floor,Floor,            // 3
            Seat,Seat,Seat,Seat,Floor,Seat,Seat,Floor,Seat,Seat,                // 4
            Seat,Floor,Seat,Seat,Floor,Seat,Seat,Floor,Seat,Seat,               // 5
            Seat,Floor,Seat,Seat,Seat,Seat,Seat,Floor,Seat,Seat,                // 6
            Floor,Floor,Seat,Floor,Seat,Floor,Floor,Floor,Floor,Floor,          // 7
            Seat,Seat,Seat,Seat,Seat,Seat,Seat,Seat,Seat,Seat,                  // 8
            Seat,Floor,Seat,Seat,Seat,Seat,Seat,Seat,Floor,Seat,                // 9
            Seat,Floor,Seat,Seat,Seat,Seat,Seat,Floor,Seat,Seat,                // 10
        ]);
    }
}