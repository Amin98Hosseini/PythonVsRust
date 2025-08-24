use std::time::Instant;

// Prime checker (simple)
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Sieve of Eratosthenes
fn sieve(n: u32) -> Vec<u32> {
    let mut is_prime_arr = vec![true; (n + 1) as usize];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;

    for i in 2..=((n as f64).sqrt() as u32) {
        if is_prime_arr[i as usize] {
            let mut j = i*i;
            while j <= n {
                is_prime_arr[j as usize] = false;
                j += i;
            }
        }
    }
    is_prime_arr.iter().enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u32) } else { None })
        .collect()
}

fn main() {
    let n = 100_000;
    let iterations = 10; // Number of times to repeat for Benchmark

    // ---------------- Simple Prime Checker Benchmark ----------------
    let start = Instant::now();
    let mut primes_simple = Vec::new();
    for _ in 0..iterations {
        primes_simple = (2..=n).filter(|&x| is_prime(x)).collect();
    }
    let duration = start.elapsed().as_micros();
    let avg_time = duration as f64 / iterations as f64;
    println!("Simple Prime Checker up to {}", n);
    println!("Number of primes: {}", primes_simple.len());
    println!("Total Runtime: {} µs", duration);
    println!("Average Runtime: {:.2} µs\n", avg_time);

    // ---------------- Sieve of Eratosthenes Benchmark ----------------
    let start = Instant::now();
    let mut primes_sieve = Vec::new();
    for _ in 0..iterations {
        primes_sieve = sieve(n);
    }
    let duration = start.elapsed().as_micros();
    let avg_time = duration as f64 / iterations as f64;
    println!("Sieve of Eratosthenes up to {}", n);
    println!("Number of primes: {}", primes_sieve.len());
    println!("Total Runtime: {} µs", duration);
    println!("Average Runtime: {:.2} µs", avg_time);
}
