import time
from csv import DictReader

start = time.perf_counter()
# https://raw.githubusercontent.com/JadenHow/Steam-Games-Recommendations/main/datasets/steam.csv
with open(
    "/home/ludo/dedomainia/talks/dedotalk/rust_for_python_dev/data/steam.csv"
) as f:
    reader = DictReader(f)
    rows = [row for row in reader]

print(rows[0])
print(time.perf_counter() - start)
