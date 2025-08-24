use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::time::Instant;
use rand::Rng;

fn dijkstra(graph: &Vec<Vec<(usize, u32)>>, start: usize) -> Vec<u32> {
    let n = graph.len();
    let mut dist = vec![u32::MAX; n];
    dist[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &graph[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                heap.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

fn main() {
    let n = 1000;
    let mut graph: Vec<Vec<(usize, u32)>> = vec![Vec::new(); n];
    let mut rng = rand::thread_rng();

    for u in 0..n {
        for _ in 0..10 {
            let v = rng.gen_range(0..n);
            let w = rng.gen_range(1..=10);
            graph[u].push((v, w));
        }
    }

    let start = Instant::now();
    let _dist = dijkstra(&graph, 0);
    let dur = start.elapsed().as_micros();
    println!("Dijkstra shortest paths computed in {} µs", dur);
}
