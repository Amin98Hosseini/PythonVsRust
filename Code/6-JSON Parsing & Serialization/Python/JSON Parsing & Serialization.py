import json
import time

# Sample data: list of dictionaries
def generate_data(n: int):
    return [{"id": i, "name": f"Item {i}", "active": i % 2 == 0, "values": list(range(10))} for i in range(n)]

def main():
    data = generate_data(100000)  # 100k objects

    # Serialization
    start = time.perf_counter()
    json_str = json.dumps(data)
    dur_ser = (time.perf_counter() - start) * 1e6
    print(f"Serialization Time: {dur_ser:.2f} µs, JSON size: {len(json_str)/1024/1024:.2f} MB")

    # Deserialization
    start = time.perf_counter()
    parsed = json.loads(json_str)
    dur_de = (time.perf_counter() - start) * 1e6
    print(f"Deserialization Time: {dur_de:.2f} µs, Objects: {len(parsed)}")

if __name__ == "__main__":
    main()
