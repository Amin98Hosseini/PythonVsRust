import random
import time

# QuickSort
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)

# MergeSort
def mergesort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = mergesort(arr[:mid])
    right = mergesort(arr[mid:])
    return merge(left, right)

def merge(left, right):
    result = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result

# --- Main ---
if __name__ == "__main__":
    size = 100000  
    arr = [random.randint(0, 100000) for _ in range(size)]

    arr1 = arr.copy()
    start = time.perf_counter()
    _sorted_quick = quicksort(arr1)
    duration_quick = (time.perf_counter() - start) * 1_000_000
    print(f"Python = QuickSort Runtime: {duration_quick:.2f} µs")

    arr2 = arr.copy()
    start = time.perf_counter()
    _sorted_merge = mergesort(arr2)
    duration_merge = (time.perf_counter() - start) * 1_000_000
    print(f"Python = MergeSort Runtime: {duration_merge:.2f} µs")
