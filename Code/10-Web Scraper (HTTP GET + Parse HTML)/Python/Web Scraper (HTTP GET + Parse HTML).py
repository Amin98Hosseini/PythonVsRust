import requests
from bs4 import BeautifulSoup
import time

def main():
    url = "https://en.wikipedia.org/wiki/Rust_(programming_language)"

    start = time.perf_counter()
    response = requests.get(url)
    dur_get = (time.perf_counter() - start) * 1e6
    print(f"HTTP GET completed in {dur_get:.2f} µs, Status Code: {response.status_code}")

    start = time.perf_counter()
    soup = BeautifulSoup(response.text, "html.parser")
    titles = [t.get_text() for t in soup.find_all("h1")]
    dur_parse = (time.perf_counter() - start) * 1e6
    print(f"HTML Parsing completed in {dur_parse:.2f} µs, Found {len(titles)} h1 tags")

    print("First h1 tag:", titles[0] if titles else "None")

if __name__ == "__main__":
    main()
