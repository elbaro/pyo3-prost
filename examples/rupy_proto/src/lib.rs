pub mod app;

use pyo3::prelude::*;

#[pymodule]
fn rupy_proto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<app::UserRef>()?;
    m.add_class::<app::TweetRef>()?;
    Ok(())
}
