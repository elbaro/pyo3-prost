use std::sync::Arc;

use pyo3::{IntoPy, PyObject};

pub mod list;

// impl AsBorrowed for Tweet
//      Borrowed = TweetRef
pub trait AsBorrowed: Sized + 'static {
    type Borrowed: Ref<Self> + IntoPy<PyObject>;

    fn as_borrowed(&self, owner: Arc<dyn Send + Sync>) -> Self::Borrowed {
        let borrowed = unsafe { std::mem::transmute::<&'_ Self, &'static Self>(self) };
        <Self::Borrowed>::new(owner, borrowed)
    }
}

pub trait Ref<Original> {
    fn new(owner: ::std::sync::Arc<dyn Send + Sync>, borrowed: &'static Original) -> Self;
    fn new_owned(owner: ::std::sync::Arc<Original>) -> Self;
}
