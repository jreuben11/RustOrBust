use pyo3::prelude::*;
use pyo3::class::basic::CompareOp;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Self(value)
    }

    // `__str__` is generally used to create an "informal" representation, so we
    // just forward to `i32`'s `ToString` trait implementation to print a bare number.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    // For `__repr__` we want to return a string that Python code could use to recreate
    // the `Number`, like `Number(5)` for example.
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        // This is the equivalent of `self.__class__.__name__` in Python.
        let class_name: String = slf.get_type().qualname()?;
        // To access fields of the Rust struct, we need to borrow the `PyCell`.
        Ok(format!("{}({})", class_name, slf.borrow().0))
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.0))
    }

    fn __bool__(&self) -> bool {
        self.0 != 0
    }
}


#[pymodule]
fn py_number(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Number>()?;
    Ok(())
}


