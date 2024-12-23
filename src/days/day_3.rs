// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    let good_deeds_f = good_deeds as f32;
    let bad_deeds_f = bad_deeds as f32;
    let ratio = (good_deeds_f * GOOD_WEIGHT) / ((good_deeds_f * GOOD_WEIGHT) + (BAD_WEIGHT * bad_deeds_f));

    if good_deeds == 0 && bad_deeds == 0 {
    false  
    } else if ratio >= 0.75 {
        true
    } else {
        false
    }
}