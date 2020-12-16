

pub type TicketNumbers = Vec<u32>;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub first_range: [u32; 2],
    pub second_range: [u32; 2],
}

#[derive(Debug, Default)]
pub struct Input {
    pub fields: Vec<Field>,
    pub my_ticket: TicketNumbers,
    pub tickets: Vec<TicketNumbers>,
}