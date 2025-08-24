use std::time::Instant;

// Recursive Fibonacci
fn fib_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

// Iterative Fibonacci
fn fib_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

fn main() {
    let n = 35;

    // Recursive runtime (single run)
    let start = Instant::now();
    let result_recursive = fib_recursive(n);
    let duration = start.elapsed().as_micros(); // µs
    println!(
        "Recursive fib({}) = {}, Runtime: {} µs",
        n, result_recursive, duration
    );

    // Iterative runtime (benchmark over 10,000 runs)
    let iterations = 10_000;
    let start = Instant::now();
    let mut result_iterative = 0;
    for _ in 0..iterations {
        result_iterative = fib_iterative(n);
    }
    let duration = start.elapsed().as_micros(); // total time in µs
    let avg_time = duration as f64 / iterations as f64;

    println!(
        "Iterative fib({}) = {}, Total Runtime: {} µs, Avg Runtime: {:.4} µs",
        n, result_iterative, duration, avg_time
    );
}
