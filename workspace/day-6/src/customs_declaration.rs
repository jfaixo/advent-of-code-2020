
// Represents a parsed group of lines
// declarations[x] => declarations of a person
// declarations[x][y] => a single question answered yes
#[derive(Debug, Default)]
pub struct CustomsDeclaration {
    pub declarations: Vec<Vec<char>>
}
