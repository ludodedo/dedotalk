from unittest import result
from csv import DictReader
import time


start = time.perf_counter()
# https://raw.githubusercontent.com/JadenHow/Steam-Games-Recommendations/main/datasets/steam.csv
with open("/path/to/steam.csv") as f:
    reader = DictReader(f)
    rows = [row for row in reader]

print(rows[0])
print(time.perf_counter()-start)
