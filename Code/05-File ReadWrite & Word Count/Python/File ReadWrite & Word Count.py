import time
import os
import string

# Function to write a large text file
def write_file(filename: str, text: str, repeat: int):
    with open(filename, "w", encoding="utf-8") as f:
        for _ in range(repeat):
            f.write(text)

# Function to read file and count words (Naive)
def word_count_naive(filename: str):
    with open(filename, "r", encoding="utf-8") as f:
        text = f.read()
    words = text.split()
    return len(words)

# Optimized word count (process line by line)
def word_count_optimized(filename: str):
    count = 0
    with open(filename, "r", encoding="utf-8") as f:
        for line in f:
            count += len(line.split())
    return count

def main():
    filename = r"D:\RustVsPython\Code\5-File ReadWrite & Word Count\Python\test_file.txt"
    sample_text = "Hello world. This is a benchmark test for file I/O and word count.\n"
    
    # Write file
    start = time.perf_counter()
    write_file(filename, sample_text, repeat=100000)  # ~5-10 MB
    dur_write = (time.perf_counter() - start) * 1e6
    print(f"File Write Time: {dur_write:.2f} µs")

    # Naive word count
    start = time.perf_counter()
    wc1 = word_count_naive(filename)
    dur_naive = (time.perf_counter() - start) * 1e6
    print(f"Naive Word Count: {wc1} words, Time: {dur_naive:.2f} µs")

    # Optimized word count
    start = time.perf_counter()
    wc2 = word_count_optimized(filename)
    dur_opt = (time.perf_counter() - start) * 1e6
    print(f"Optimized Word Count: {wc2} words, Time: {dur_opt:.2f} µs")

    os.remove(filename)

if __name__ == "__main__":
    main()
