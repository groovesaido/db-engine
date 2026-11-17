use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
    let mut scores = BTreeMap::new();
    scores.insert(3, "zoe");
    scores.insert(5, "ken");
    println!("{:?}", scores);
}
