use std::thread;
use std::sync::mpsc;
use std::time::Instant;

fn main() {
    let n: u64 = 10_000_000;
    let threads = 4;
    let chunk = n / threads;

    // Channel برای جمع نتایج از هر thread
    let (tx, rx) = mpsc::channel();

    // --- Threading (True multithreading) ---
    let start = Instant::now();
    let mut handles = vec![];

    for i in 0..threads {
        let t_start = i * chunk + 1;
        let t_end = if i == threads - 1 { n } else { (i + 1) * chunk };
        let handle = thread::spawn(move || (t_start..=t_end).sum::<u64>());
        handles.push(handle);
    }

    let mut total_threading = 0;
    for h in handles {
        total_threading += h.join().unwrap();
    }

    let dur_threading = start.elapsed().as_micros();
    println!("Rust Threading Total: {}, Time: {} µs", total_threading, dur_threading);

    // --- “Multiprocessing” style with threads + channel ---
    let start = Instant::now();
    let mut handles_mp = vec![];

    for i in 0..threads {
        let t_start = i * chunk + 1;
        let t_end = if i == threads - 1 { n } else { (i + 1) * chunk };
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let s: u64 = (t_start..=t_end).sum();
            tx_clone.send(s).unwrap(); // ارسال نتیجه به channel
        });
        handles_mp.push(handle);
    }

    // جمع نتایج از channel
    let mut total_mp = 0;
    for _ in 0..threads {
        total_mp += rx.recv().unwrap();
    }

    for h in handles_mp {
        h.join().unwrap();
    }

    let dur_mp = start.elapsed().as_micros();
    println!("Rust Multiprocessing-style Total: {}, Time: {} µs", total_mp, dur_mp);
}
