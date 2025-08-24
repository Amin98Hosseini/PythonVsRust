use rand::Rng;  
use rand::rng;  

// QuickSort
fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let pivot = arr[arr.len() / 2];
    let mut left: Vec<i32> = arr.iter().cloned().filter(|&x| x < pivot).collect();
    let middle: Vec<i32> = arr.iter().cloned().filter(|&x| x == pivot).collect();
    let mut right: Vec<i32> = arr.iter().cloned().filter(|&x| x > pivot).collect();

    left = quicksort(left);
    right = quicksort(right);

    let mut result = Vec::new();
    result.extend(left);
    result.extend(middle);
    result.extend(right);
    result
}

// MergeSort
fn mergesort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;
    let left = mergesort(arr[..mid].to_vec());
    let right = mergesort(arr[mid..].to_vec());

    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

use std::time::Instant;

fn main() {
    let size = 100000; 
    let mut rng = rng(); 

    let arr: Vec<i32> = (0..size).map(|_| rng.random_range(0..100000)).collect();

    let arr1 = arr.clone();
    let start = Instant::now();
    let _sorted_quick = quicksort(arr1);
    let duration_quick = start.elapsed();
    println!("Rust = QuickSort Runtime: {} µs", duration_quick.as_micros());

    let arr2 = arr.clone();
    let start = Instant::now();
    let _sorted_merge = mergesort(arr2);
    let duration_merge = start.elapsed();
    println!("Rust = MergeSort Runtime: {} µs", duration_merge.as_micros());
}
