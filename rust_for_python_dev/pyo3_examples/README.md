Build and install in the local virtual env `maturin develop` and in release `maturin develop --release`


Then inside python
```python
from pyo3_examples import compute_fibonacci_number
compute_fibonacci_number(50) # 12586269025
```

Things to try:
- Passing a negative number raise a python exeption
- compute_fibonacci_number(94) panic in debug mode due to overflow (max u64 18_446_744_073_709_551_615)
