import heapq
import time

def dijkstra(graph, start):
    n = len(graph)
    dist = [float('inf')] * n
    dist[start] = 0
    pq = [(0, start)]  # priority queue: (distance, node)

    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in graph[u]:
            if dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                heapq.heappush(pq, (dist[v], v))
    return dist

def main():
    n = 1000  # تعداد نودها
    graph = [[] for _ in range(n)]
    import random
    for u in range(n):
        for _ in range(10):  # هر نود 10 یال تصادفی
            v = random.randint(0, n-1)
            w = random.randint(1, 10)
            graph[u].append((v, w))

    start = time.perf_counter()
    dist = dijkstra(graph, 0)
    dur = (time.perf_counter() - start) * 1e6
    print(f"Dijkstra shortest paths computed in {dur:.2f} µs")

if __name__ == "__main__":
    main()
