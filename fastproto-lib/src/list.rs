use std::sync::Arc;

use pyo3::{pyclass, pymethods, IntoPy, PyObject, Python};

use crate::AsBorrowed;

pub struct BorrowedList<Item: 'static> {
    // Item=Tweet, Borrowed=TweetBorrowed
    owner: Arc<dyn Send + Sync>,
    slice: &'static [Item],
}

#[pyclass]
pub struct ProxyList(Box<dyn ProxyListTrait + Send + Sync>);

#[pymethods]
impl ProxyList {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get<'py>(&self, idx: usize, py: Python<'py>) -> PyObject {
        self.0.get(idx, py)
    }
    pub fn iter(&self) {}
}

pub trait ProxyListTrait {
    fn len(&self) -> usize;
    fn get<'py>(&self, idx: usize, py: Python<'py>) -> PyObject;
    fn iter(&self);
}

impl<Item: AsBorrowed> ProxyListTrait for BorrowedList<Item> {
    fn len(&self) -> usize {
        self.slice.len()
    }
    fn get<'py>(&self, idx: usize, py: Python<'py>) -> PyObject {
        // &Tweet -> TweetRef { owner, borrowed}
        self.slice[idx].as_borrowed(self.owner.clone()).into_py(py)
    }
    fn iter(&self) {}
}
