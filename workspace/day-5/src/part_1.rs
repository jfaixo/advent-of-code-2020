use crate::boarding_pass::BoardingPass;

pub fn highest_seat_id(boarding_passes: &Vec<BoardingPass>) -> u32 {
    boarding_passes.iter()
        .map(|boarding_pass| {
            boarding_pass.seat_id()
        })
        .max()
        .expect("Error in part 1")
}