
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LayoutPosition {
    Floor,
    Seat,
}

#[derive(Debug)]
pub struct SitLayout {
    pub width: usize,
    pub height: usize,
    pub layout: Vec<LayoutPosition>
}