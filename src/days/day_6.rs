// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_count = s1.trim().chars().count();
    let s2_count = s2.trim().chars().count();
    if s1_count > s2_count {
        Some(s1)
    } else if s1_count < s2_count {
        Some(s2)
    } else {
        None
    }
}