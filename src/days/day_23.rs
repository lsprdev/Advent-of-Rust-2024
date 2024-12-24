use std::collections::HashMap;
#[derive(Default)]
pub struct SantaList {
    records: HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        Self::default()
    }

    // 3. Implement the add method
    pub fn add(&mut self, name: &str, is_nice: bool) {
        let _ = self.records.insert(name.to_string(), is_nice);
    }
    // 4. Implement the remove method
    pub fn remove(&mut self, name: &str) {
        self.records.remove(name);
    }
    // 5. Implement the get method
    pub fn get(&self, name: &str) -> Option<bool> {
        self.records.get(name).copied()
    }
    // 6. Implement the count method
    pub fn count(&self) -> (usize, usize) {
        (
            self.records.values().filter(|&&v| v).count(),
            self.records.values().filter(|&&v| !v).count(),
        )
    }
    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, nice: bool) -> Vec<&str> {
        self.records.iter().filter_map(|(k, &v)| if v == nice { Some(k.as_ref()) } else { None }).collect()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
