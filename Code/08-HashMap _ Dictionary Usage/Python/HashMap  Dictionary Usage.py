import time

def main():
    n = 1_000_000  # Number of Otems
    value = 1  # i32 Fixed Value

    # --- Insert ---
    start = time.perf_counter()
    d = {}
    for i in range(n):
        d[i] = value
    dur_insert = (time.perf_counter() - start) * 1e6
    print(f"Python Insert {n} items: {dur_insert:.2f} µs")

    # --- Search ---
    start = time.perf_counter()
    found = all(d.get(i) is not None for i in range(n))
    dur_search = (time.perf_counter() - start) * 1e6
    print(f"Python Search {n} items: {dur_search:.2f} µs, Found: {found}")

    # --- Delete ---
    start = time.perf_counter()
    for i in range(n):
        del d[i]
    dur_delete = (time.perf_counter() - start) * 1e6
    print(f"Python Delete {n} items: {dur_delete:.2f} µs")

if __name__ == "__main__":
    main()
