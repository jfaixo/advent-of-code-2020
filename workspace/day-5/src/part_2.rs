use crate::boarding_pass::BoardingPass;

pub fn my_seat_id(boarding_passes: &mut Vec<BoardingPass>) -> u32 {
    boarding_passes.sort_by_key(|boarding_pass| boarding_pass.seat_id());

    let initial_value = boarding_passes[0].seat_id();
    for i in 1..boarding_passes.len() - 1 {
        if (initial_value + i as u32)  != boarding_passes[i].seat_id() {
            return initial_value + i as u32;
        }
    }
    0
}