pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String, 
    pub niceness: Niceness,
}

// Move yesterday's function to an associated function in the struct
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }

    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

    let ratio = good_deeds / (good_deeds + bad_deeds);

    ratio >= 0.75
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        Kid {
            name,
            niceness: if is_nice(good_deeds, bad_deeds) {
                Niceness::Nice(good_deeds)
            } else {
                Niceness::Naughty
            }
        }
    }
}

fn main() {
    let kids = vec![
        Kid::new("Alice".to_string(), 3, 1),
        Kid::new("Bob".to_string(), 1, 3),
        Kid::new("Charlie".to_string(), 10, 2),
    ];

    for kid in kids {
        println!("{:?} is {:?}", kid.name, kid.niceness);
    }
}