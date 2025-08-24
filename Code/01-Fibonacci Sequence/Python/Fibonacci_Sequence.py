import time

# Recursive Fibonacci
def fib_recursive(n):
    if n <= 1:
        return n
    return fib_recursive(n - 1) + fib_recursive(n - 2)

# Iterative Fibonacci
def fib_iterative(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a

n = 35

# Recursive runtime (single run, because it's slow)
start = time.perf_counter()
result_recursive = fib_recursive(n)
end = time.perf_counter()
runtime_us = (end - start) * 1_000_000
print(f"Recursive fib({n}) = {result_recursive}, Runtime: {runtime_us:.2f} µs")

# Iterative runtime (benchmark over 10,000 runs)
iterations = 10_000
start = time.perf_counter()
for _ in range(iterations):
    result_iterative = fib_iterative(n)
end = time.perf_counter()

total_runtime_us = (end - start) * 1_000_000  # total in µs
avg_runtime_us = total_runtime_us / iterations
print(f"Iterative fib({n}) = {result_iterative}, "
      f"Total Runtime: {total_runtime_us:.2f} µs, "
      f"Avg Runtime: {avg_runtime_us:.6f} µs")
