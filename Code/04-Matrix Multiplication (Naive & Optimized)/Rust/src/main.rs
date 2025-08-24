use std::time::Instant;

// Naive O(n^3) matrix multiplication
fn matmul_naive(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

// Optimized version: transpose b first
fn matmul_optimized(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    // Transpose b
    let mut b_t = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b_t[i][j] = b[j][i];
        }
    }

    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s += a[i][k] * b_t[j][k];
            }
            result[i][j] = s;
        }
    }
    result
}

fn main() {
    let n = 300;
    let a: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| (i + j) as f64).collect())
        .collect();
    let b: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| (i * j) as f64).collect())
        .collect();

    // Naive
    let start = Instant::now();
    let _res1 = matmul_naive(&a, &b);
    let dur1 = start.elapsed().as_micros();
    println!("Rust Naive Matrix Multiplication: {} µs", dur1);

    // Optimized
    let start = Instant::now();
    let _res2 = matmul_optimized(&a, &b);
    let dur2 = start.elapsed().as_micros();
    println!("Rust Optimized Matrix Multiplication: {} µs", dur2);
}
