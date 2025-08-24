use serde::{Serialize, Deserialize};
use serde_json;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct Item {
    id: usize,
    name: String,
    active: bool,
    values: Vec<i32>,
}

// Generate test data
fn generate_data(n: usize) -> Vec<Item> {
    (0..n).map(|i| Item {
        id: i,
        name: format!("Item {}", i),
        active: i % 2 == 0,
        values: (0..10).collect(),
    }).collect()
}

fn main() {
    let data = generate_data(100000); // 100k objects

    // Serialization
    let start = Instant::now();
    let json_str = serde_json::to_string(&data).unwrap();
    let dur_ser = start.elapsed().as_micros();
    println!("Serialization Time: {} µs, JSON size: {:.2} MB", dur_ser, json_str.len() as f64 / 1024.0 / 1024.0);

    // Deserialization
    let start = Instant::now();
    let parsed: Vec<Item> = serde_json::from_str(&json_str).unwrap();
    let dur_de = start.elapsed().as_micros();
    println!("Deserialization Time: {} µs, Objects: {}", dur_de, parsed.len());
}
