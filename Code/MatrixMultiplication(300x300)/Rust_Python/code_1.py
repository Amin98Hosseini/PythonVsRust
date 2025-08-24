import time
import random

def matmul(A, B):
    n = len(A)
    m = len(B[0])
    p = len(B)
    result = [[0.0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            s = 0.0
            for k in range(p):
                s += A[i][k] * B[k][j]
            result[i][j] = s
    return result

if __name__ == "__main__":
    N = 300  # 300x300 matrix multiplication
    A = [[random.random() for _ in range(N)] for _ in range(N)]
    B = [[random.random() for _ in range(N)] for _ in range(N)]

    start = time.time()
    C = matmul(A, B)
    end = time.time()

    print(f"Matrix {N}x{N} multiplied.")
    print(f"Sample result[0][0] = {C[0][0]:.4f}")
    print(f"Python runtime: {end - start:.4f} seconds")
