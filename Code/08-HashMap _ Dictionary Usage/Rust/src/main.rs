use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let n = 1_000_000;
    let value = 1; // i32 Fixed Value

    // --- Insert ---
    let start = Instant::now();
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(i, value);
    }
    let dur_insert = start.elapsed().as_micros();
    println!("Rust Insert {} items: {} µs", n, dur_insert);

    // --- Search ---
    let start = Instant::now();
    let mut found = true;
    for i in 0..n {
        if map.get(&i).is_none() {
            found = false;
            break;
        }
    }
    let dur_search = start.elapsed().as_micros();
    println!("Rust Search {} items: {} µs, Found: {}", n, dur_search, found);

    // --- Delete ---
    let start = Instant::now();
    for i in 0..n {
        map.remove(&i);
    }
    let dur_delete = start.elapsed().as_micros();
    println!("Rust Delete {} items: {} µs", n, dur_delete);
}
