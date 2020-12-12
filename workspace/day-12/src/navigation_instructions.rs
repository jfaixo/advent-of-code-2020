
#[derive(Debug, Eq, PartialEq)]
pub enum NavigationAction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug, Eq, PartialEq)]
pub struct NavigationInstruction {
    pub action: NavigationAction,
    pub value: i32,
}

pub type NavigationInstructions = Vec<NavigationInstruction>;
