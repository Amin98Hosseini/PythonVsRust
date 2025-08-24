import threading
import multiprocessing
import time

def sum_range(start, end, result, index):
    s = sum(range(start, end))
    result[index] = s

def main():
    N = 10_000_000
    THREADS = 4
    chunk = N // THREADS
    threads = []
    results = [0] * THREADS

    # --- Multithreading (GIL limited) ---
    start = time.perf_counter()
    for i in range(THREADS):
        t_start = i * chunk
        t_end = N if i == THREADS - 1 else (i + 1) * chunk
        t = threading.Thread(target=sum_range, args=(t_start, t_end, results, i))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()
    total_threading = sum(results)
    dur_threading = (time.perf_counter() - start) * 1e6
    print(f"Multithreading Total: {total_threading}, Time: {dur_threading:.2f} µs")

    # --- Multiprocessing (True parallel in Python) ---
    start = time.perf_counter()
    manager = multiprocessing.Manager()
    results_mp = manager.list([0] * THREADS)
    processes = []

    for i in range(THREADS):
        t_start = i * chunk
        t_end = N if i == THREADS - 1 else (i + 1) * chunk
        p = multiprocessing.Process(target=sum_range, args=(t_start, t_end, results_mp, i))
        processes.append(p)
        p.start()

    for p in processes:
        p.join()
    total_mp = sum(results_mp)
    dur_mp = (time.perf_counter() - start) * 1e6
    print(f"Multiprocessing Total: {total_mp}, Time: {dur_mp:.2f} µs")

if __name__ == "__main__":
    main()
