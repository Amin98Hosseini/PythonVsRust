use std::fs::{File, remove_file};
use std::io::{Write, BufReader, BufRead, Read};
use std::time::Instant;

// Write a large text file
fn write_file(filename: &str, text: &str, repeat: usize) {
    let mut file = File::create(filename).unwrap();
    for _ in 0..repeat {
        file.write_all(text.as_bytes()).unwrap();
    }
}

// Naive word count: read whole file at once
fn word_count_naive(filename: &str) -> usize {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split_whitespace().count()
}

// Optimized word count: line by line with BufReader
fn word_count_optimized(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        count += l.split_whitespace().count();
    }
    count
}

fn main() {
    let filename = r"D:\RustVsPython\Code\5-File ReadWrite & Word Count\Python\test_file.txt";
    let sample_text = "Hello world. This is a benchmark test for file I/O and word count.\n";

    // Write file
    let start = Instant::now();
    write_file(filename, sample_text, 100000); // ~5-10 MB
    let dur_write = start.elapsed().as_micros();
    println!("File Write Time: {} µs", dur_write);

    // Naive word count
    let start = Instant::now();
    let wc1 = word_count_naive(filename);
    let dur_naive = start.elapsed().as_micros();
    println!("Naive Word Count: {} words, Time: {} µs", wc1, dur_naive);

    // Optimized word count
    let start = Instant::now();
    let wc2 = word_count_optimized(filename);
    let dur_opt = start.elapsed().as_micros();
    println!("Optimized Word Count: {} words, Time: {} µs", wc2, dur_opt);

    remove_file(filename).unwrap();
}
