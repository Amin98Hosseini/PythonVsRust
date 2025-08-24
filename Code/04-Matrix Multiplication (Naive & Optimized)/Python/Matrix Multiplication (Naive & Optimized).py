import time

# Naive O(n^3) matrix multiplication
def matmul_naive(a, b):
    n = len(a)
    result = [[0.0 for _ in range(n)] for _ in range(n)]
    for i in range(n):
        for j in range(n):
            for k in range(n):
                result[i][j] += a[i][k] * b[k][j]
    return result

# Optimized version: transpose B first for better cache locality
def matmul_optimized(a, b):
    n = len(a)
    # Transpose b for faster access
    b_t = [[b[j][i] for j in range(n)] for i in range(n)]
    result = [[0.0 for _ in range(n)] for _ in range(n)]
    for i in range(n):
        for j in range(n):
            s = 0.0
            for k in range(n):
                s += a[i][k] * b_t[j][k]
            result[i][j] = s
    return result

def main():
    n = 300  # Matrix size
    a = [[float(i + j) for j in range(n)] for i in range(n)]
    b = [[float(i * j) for j in range(n)] for i in range(n)]

    # Naive version
    start = time.perf_counter()
    _res1 = matmul_naive(a, b)
    dur1 = (time.perf_counter() - start) * 1e6
    print(f"python Naive Matrix Multiplication: {dur1:.2f} µs")

    # Optimized version
    start = time.perf_counter()
    _res2 = matmul_optimized(a, b)
    dur2 = (time.perf_counter() - start) * 1e6
    print(f"python Optimized Matrix Multiplication: {dur2:.2f} µs")

if __name__ == "__main__":
    main()
