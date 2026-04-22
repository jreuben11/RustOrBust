use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::attach(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let user: String = py
            .eval(
                c"os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'",
                None,
                Some(&locals),
            )?
            .extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
