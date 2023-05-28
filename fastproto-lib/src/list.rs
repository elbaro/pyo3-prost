use std::sync::Arc;

use pyo3::{exceptions::PyIndexError, pyclass, pymethods, IntoPy, PyObject, PyResult, Python};

use crate::AsBorrowed;

pub struct BorrowedList<Item: 'static> {
    // Item=Tweet, Borrowed=TweetBorrowed
    pub owner: Arc<dyn Send + Sync>,
    pub slice: &'static [Item],
}

#[pyclass]
pub struct ProxyList(pub Box<dyn ProxyListTrait + Send + Sync>);

#[pymethods]
impl ProxyList {
    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.0.len())
    }
    pub fn __getitem__<'py>(&self, idx: isize, py: Python<'py>) -> PyResult<PyObject> {
        self.0.get(idx, py)
    }
    pub fn iter(&self) {}
}

pub trait ProxyListTrait {
    fn len(&self) -> usize;
    fn get<'py>(&self, idx: isize, py: Python<'py>) -> PyResult<PyObject>;
    fn iter(&self);
}

impl<Item: AsBorrowed> ProxyListTrait for BorrowedList<Item> {
    fn len(&self) -> usize {
        self.slice.len()
    }
    fn get<'py>(&self, idx: isize, py: Python<'py>) -> PyResult<PyObject> {
        // &Tweet -> TweetRef { owner, borrowed}
        let idx = if idx >= 0 {
            if idx < self.slice.len() as isize {
                idx as usize
            } else {
                return Err(PyIndexError::new_err("list index out of range"));
            }
        } else {
            if self.slice.len() as isize + idx >= 0 {
                (self.slice.len() as isize + idx) as usize
            } else {
                return Err(PyIndexError::new_err("list index out of range"));
            }
        };
        Ok(self.slice[idx].as_borrowed(self.owner.clone()).into_py(py))
    }
    fn iter(&self) {}
}
