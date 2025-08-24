# 🚀 Python vs Rust Performance Benchmarks  

This repository contains **10 performance benchmark programs** implemented in **Python** and **Rust**.  
The goal is to compare execution speed, memory efficiency, and concurrency capabilities between Python and Rust in different scenarios.  

---

## 📂 Benchmarks Included  

### 1️⃣ Fibonacci (Recursive & Iterative)
- Compare recursive vs iterative Fibonacci number calculation.  
- Tests stack usage & recursion performance.  

### 2️⃣ Prime Number Calculation  
- Generate prime numbers up to N.  
- Useful for CPU-bound number crunching.  

### 3️⃣ Sorting Algorithms (QuickSort & MergeSort)  
- Compare recursive sorting algorithms.  
- Good for memory-intensive workloads.  

### 4️⃣ Matrix Multiplication (Naive & Optimized)  
- Implement naive O(n³) matrix multiplication.  
- Optimized version uses loop reordering / blocking.  
- Numerical computing performance test.  

### 5️⃣ File Read/Write & Word Count  
- Write large text files, read them, and count words.  
- Compares I/O speed and string manipulation.  

### 6️⃣ JSON Parsing & Serialization  
- Serialize and parse JSON data.  
- Python → `json`  
- Rust → `serde_json`  

### 7️⃣ Multithreading / Concurrency  
- Sum of numbers using:  
  - Python → Multithreading (GIL-limited) & Multiprocessing  
  - Rust → True multithreading + multiprocessing  

### 8️⃣ HashMap / Dictionary Usage  
- Insert, search, and delete 1,000,000 key-value pairs.  
- Compare performance of Python `dict` vs Rust `HashMap`.  

### 9️⃣ Pathfinding Algorithm (Dijkstra)  
- Implement Dijkstra’s shortest path.  
- Compare graph traversal efficiency.  

### 🔟 Web Scraper (HTTP GET + HTML Parse)  
- Python → `requests + BeautifulSoup`  
- Rust → `reqwest + scraper`  
- Extract links from a webpage.  

---

## 🛠 Requirements  

### Python  
- Python 3.10+  
- Required libraries:  
  ```bash
  pip install requests beautifulsoup4
  ```


### Rust  
- Rust 1.80+
- Required crates (added via cargo add):
  ```toml
  cargo add rand
  cargo add serde_json
  cargo add reqwest
  cargo add scraper
  cargo add tokio --features full
  ```




## 🎯 Conclusion

- **Rust** is consistently faster in CPU-bound tasks, recursion, and multithreading.  
- **Python** is easier to write and has powerful libraries, but suffers from the GIL and slower execution.  

This repo is a learning experiment to demonstrate how both languages handle common workloads.

---

## 📌 Author

Created by **Mohammad Amin** ✨

## Support Me
<a href="https://www.coffeebede.com/amin98hosseini">
  <img 
    class="img-fluid" 
    src="https://coffeebede.ir/DashboardTemplateV2/app-assets/images/banner/default-yellow.svg" 
    width="150" 
    height="50" 
    alt="Buy Me a Coffee" 
  />
</a>