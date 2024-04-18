# quickstart_r2p
## pyenv config and maturin 
```bash
conda deactivate # IMPORTANT - cant have 2 envs.
pyenv versions
pyenv virtualenv 3.11.7 Py03
pyenv local Py03
pyenv which python 
python3.11 -m pip install --upgrade pip
pip install maturin
mkdir xxx && cd xxx
maturin init --bindings pyo3
maturin develop
python src/tester.py
```
## [Cargo.toml](quickstart_r2p/Cargo.toml)
```toml
[lib]
name = "quickstart_r2p" # IMPORTANT !!! python module to generate
crate-type = ["cdylib"]

[dependencies]
pyo3 = "0.21.2"

[workspace] # IMPORTANT !!! Maturin does not like parent workspaces
```
## [lib.rs](quickstart_r2p/src/lib.rs)
```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn quickstart_r2p(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```
## [tester.py](quickstart_r2p/src/tester.py)
```python
import quickstart_r2p
sum = quickstart_r2p.sum_as_string(5, 20)
print(f"sum: {sum}")
```
