#![feature(macro_attributes_in_derive_output)]

pub mod app;

use pyo3::prelude::*;

#[pymodule]
fn rupy_proto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<app::UserOwned>()?;
    m.add_class::<app::TweetOwned>()?;
    Ok(())
}
