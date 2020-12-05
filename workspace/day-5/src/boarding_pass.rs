
#[derive(Debug, Default, PartialEq)]
pub struct BoardingPass {
    row: u32,
    column: u32,
}

impl BoardingPass {
    pub fn from_encoded_boarding_pass(encoded_boarding_pass: &str) -> BoardingPass {
        let mut boarding_pass: BoardingPass = Default::default();

        let mut iterator = encoded_boarding_pass.chars();
        // Parse the row
        for _i in 0..7 {
            boarding_pass.row = boarding_pass.row << 1;
            let char = iterator.next().expect("Incorrect format");
            if char == 'B' {
                boarding_pass.row += 1
            }
        }
        // Parse the column
        for _i in 0..3 {
            boarding_pass.column = boarding_pass.column << 1;
            let char = iterator.next().expect("Incorrect format");
            if char == 'R' {
                boarding_pass.column += 1
            }
        }

        boarding_pass
    }

    pub fn seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn parse_encoded_boarding_pass() {
        let mut pass = BoardingPass::from_encoded_boarding_pass("FFFFFFFLLL");
        assert_eq!(pass, BoardingPass { row: 0, column: 0 });

        pass = BoardingPass::from_encoded_boarding_pass("FBFBBFFRLR");
        assert_eq!(pass, BoardingPass { row: 44, column: 5 });

        pass = BoardingPass::from_encoded_boarding_pass("BBBBBBBRRR");
        assert_eq!(pass, BoardingPass { row: 127, column: 7 });
    }
}