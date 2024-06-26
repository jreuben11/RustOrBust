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

# quickstart_p2r
- bug: see https://github.com/PyO3/pyo3/issues/2803
- workaround: `export LD_LIBRARY_PATH=${HOME}/.pyenv/versions/3.11.7/lib:$LD_LIBRARY_PATH`
## [Cargo.toml](quickstart_p2r/Cargo.toml)
```toml
[dependencies]
pyo3 = { version = "0.21.2", features = ["auto-initialize"] }

[workspace]
```
## [main.rs](quickstart_p2r/src/main.rs)
```rust
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
```

# py_number
- [lib.rs](py_number/src/lib.rs)
```rust
use pyo3::prelude::*;

#[pyclass]
struct Number(i32);

fn wrap(obj: &Bound<'_, PyAny>) -> PyResult<i32> { ... }

#[pymethods]
impl Number {
    #[new]
    fn new(#[pyo3(from_py_with = "wrap")] value: i32) -> Self { ... }
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> { ... }
    fn __str__(&self) -> String { ... }
    fn __hash__(&self) -> u64 { ... }
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> { ... }
    fn __bool__(&self) -> bool { ... }
    fn __add__(&self, other: &Self) -> Self { ... }
}

#[pyclass(name = "Counter")]
pub struct PyCounterWrapper {
    count: Cell<u64>,
    wraps: Py<PyAny>,
}

#[pymethods]
impl PyCounterWrapper {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self { ... }
    #[getter]
    fn count(&self) -> u64 { ... }
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(&self, py: Python<'_>, args: &Bound<'_, PyTuple>, kwargs: Option<&Bound<'_, PyDict>>,) -> PyResult<Py<PyAny>> { ... }
}

#[pymodule]
fn py_number(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Number>()?;
    m.add_class::<PyCounterWrapper>()?;
    Ok(())
}

```
- [pytester](py_number/src/tester.py)

