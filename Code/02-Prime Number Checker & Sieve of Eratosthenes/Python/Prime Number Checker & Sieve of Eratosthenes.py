import math
import time

# ---------------- Simple Prime Checker ----------------
def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True

# ---------------- Sieve of Eratosthenes ----------------
def sieve(n):
    is_prime_arr = [True] * (n + 1)
    is_prime_arr[0] = is_prime_arr[1] = False

    for i in range(2, int(math.sqrt(n)) + 1):
        if is_prime_arr[i]:
            for j in range(i*i, n + 1, i):
                is_prime_arr[j] = False

    primes = [i for i, val in enumerate(is_prime_arr) if val]
    return primes

# ---------------- Benchmark ----------------
n = 100_000
iterations = 10  # Number of times to repeat for Benchmark

# Simple Prime Checker Benchmark
start = time.perf_counter()
primes_simple = []
for _ in range(iterations):
    primes_simple = [i for i in range(2, n+1) if is_prime(i)]
end = time.perf_counter()
total_runtime_us = (end - start) * 1_000_000
avg_runtime_us = total_runtime_us / iterations
print(f"Simple Prime Checker up to {n}")
print(f"Number of primes: {len(primes_simple)}")
print(f"Total Runtime: {total_runtime_us:.2f} µs")
print(f"Average Runtime: {avg_runtime_us:.2f} µs\n")

# Sieve of Eratosthenes Benchmark
start = time.perf_counter()
primes_sieve = []
for _ in range(iterations):
    primes_sieve = sieve(n)
end = time.perf_counter()
total_runtime_us = (end - start) * 1_000_000
avg_runtime_us = total_runtime_us / iterations
print(f"Sieve of Eratosthenes up to {n}")
print(f"Number of primes: {len(primes_sieve)}")
print(f"Total Runtime: {total_runtime_us:.2f} µs")
print(f"Average Runtime: {avg_runtime_us:.2f} µs")
