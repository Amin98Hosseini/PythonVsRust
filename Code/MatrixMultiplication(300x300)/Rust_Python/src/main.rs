use rand::Rng;
use std::time::Instant;

fn matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();

    let mut result = vec![vec![0.0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let mut s = 0.0;
            for k in 0..p {
                s += a[i][k] * b[k][j];
            }
            result[i][j] = s;
        }
    }

    result
}

fn main() {
    let n = 300; // 300x300 matrix multiplication
    let mut rng = rand::rng(); 

    let a: Vec<Vec<f64>> = (0..n)
        .map(|_| (0..n).map(|_| rng.random::<f64>()).collect())
        .collect();

    let b: Vec<Vec<f64>> = (0..n)
        .map(|_| (0..n).map(|_| rng.random::<f64>()).collect())
        .collect();

    let start = Instant::now();
    let c = matmul(&a, &b);
    let duration = start.elapsed();

    println!("Matrix {}x{} multiplied.", n, n);
    println!("Sample result[0][0] = {:.4}", c[0][0]);
    println!("Rust runtime: {:?}", duration);
}
