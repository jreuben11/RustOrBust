use pyo3::prelude::*;
use pyo3::class::basic::CompareOp;
use pyo3::types::PyComplex;
use pyo3::exceptions::{PyZeroDivisionError, PyValueError};
use pyo3::ffi;

use std::os::raw::c_ulong;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[pyclass]
struct Number(i32);

fn wrap(obj: &Bound<'_, PyAny>) -> PyResult<i32> {
    // let val = obj.call_method1("__and__", (0xFFFFFFFF_u32,))?;
    // let val: u32 = val.extract()?;
    // //     👇 This intentionally overflows!
    // Ok(val as i32)
    let py: Python<'_> = obj.py();

    unsafe {
        let ptr = obj.as_ptr();

        let ret: c_ulong = ffi::PyLong_AsUnsignedLongMask(ptr);
        if ret == c_ulong::MAX {
            if let Some(err) = PyErr::take(py) {
                return Err(err);
            }
        }

        Ok(ret as i32)
    }
}

#[pymethods]
impl Number {
    #[new]
    fn new(#[pyo3(from_py_with = "wrap")] value: i32) -> Self {
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

    fn __add__(&self, other: &Self) -> Self {
        Self(self.0.wrapping_add(other.0))
    }

    fn __sub__(&self, other: &Self) -> Self {
        Self(self.0.wrapping_sub(other.0))
    }

    fn __mul__(&self, other: &Self) -> Self {
        Self(self.0.wrapping_mul(other.0))
    }

    fn __truediv__(&self, other: &Self) -> PyResult<Self> {
        match self.0.checked_div(other.0) {
            Some(i) => Ok(Self(i)),
            None => Err(PyZeroDivisionError::new_err("division by zero")),
        }
    }

    fn __floordiv__(&self, other: &Self) -> PyResult<Self> {
        match self.0.checked_div(other.0) {
            Some(i) => Ok(Self(i)),
            None => Err(PyZeroDivisionError::new_err("division by zero")),
        }
    }

    fn __rshift__(&self, other: &Self) -> PyResult<Self> {
        match other.0.try_into() {
            Ok(rhs) => Ok(Self(self.0.wrapping_shr(rhs))),
            Err(_) => Err(PyValueError::new_err("negative shift count")),
        }
    }

    fn __lshift__(&self, other: &Self) -> PyResult<Self> {
        match other.0.try_into() {
            Ok(rhs) => Ok(Self(self.0.wrapping_shl(rhs))),
            Err(_) => Err(PyValueError::new_err("negative shift count")),
        }
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }

    fn __invert__(&self) -> Self {
        Self(!self.0)
    }

    fn __int__(&self) -> i32 {
        self.0
    }

    fn __float__(&self) -> f64 {
        self.0 as f64
    }

    fn __complex__<'py>(&self, py: Python<'py>) -> Bound<'py, PyComplex> {
        PyComplex::from_doubles_bound(py, self.0 as f64, 0.0)
    }
}


use pyo3::types::{PyDict, PyTuple};
use std::cell::Cell;

/// A function decorator that keeps track how often it is called.
#[pyclass(name = "Counter")]
pub struct PyCounter {
    // Keeps track of how many calls have gone through.
    count: Cell<u64>,
    // This is the actual function being wrapped.
    wraps: Py<PyAny>,
}

#[pymethods]
impl PyCounter {
    // Note that we don't validate whether `wraps` is actually callable.
    //
    // While we could use `PyAny::is_callable` for that, it has some flaws:
    //    1. It doesn't guarantee the object can actually be called successfully
    //    2. We still need to handle any exceptions that the function might raise
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        PyCounter {
            count: Cell::new(0),
            wraps,
        }
    }

    #[getter]
    fn count(&self) -> u64 {
        self.count.get()
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let old_count = self.count.get();
        let new_count = old_count + 1;
        self.count.set(new_count);
        let name = self.wraps.getattr(py, "__name__")?;

        println!("{} has been called {} time(s).", name, new_count);

        // After doing something, we finally forward the call to the wrapped function
        let ret = self.wraps.call_bound(py, args, kwargs)?;

        // We could do something with the return value of
        // the function before returning it
        Ok(ret)
    }
}


#[pymodule]
fn py_number(_py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Number>()?;
    module.add_class::<PyCounter>()?;
    Ok(())
}


