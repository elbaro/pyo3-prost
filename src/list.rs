// ## List<T: ToPyObject>
// TODO: move to separate lib

pub struct BorrowedList<Item, Borrowed> {
    // Item=Tweet, Borrowed=TweetBorrowed
    owner: Arc<dyn Any + Send + Sync>,
    slice: &'static [Item],
}

#[pyclass]
pub struct ProxyList(Box<dyn ProxyListTrait>);

#[pymethod]
impl ProxyList {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, idx: usize) -> &PyAny {
        self.0.get(idx)
    }
    pub fn iter() {}
}

pub trait ProxyListTrait {
    fn len(&self) -> usize;
    fn get<'py>(&self, idx: usize, py: Python<'py>);
    fn iter();
}

// impl ProxyListTrait for [i64] {}
// impl ProxyListTrait for [i32] {}

// impl ProxyListTrait for BorrowedList<Item, Borrowed> {
//     fn len(&self) -> usize {
//         self.slice.len()
//     }
//     fn get<'py>(&self, idx: usize, py: Python<'py>) -> &'py PyAny {
//         self.slice[i]
//         Borrowed {
//             owner: ,
//             borrowed: ,
//         }
//     }
//     // fn slice(&self, selector: ()) -> Box<dyn ProxyListTrait> {
//     //   self.0.clone().map(|slice| slice[selector])
//     //    Box::new(BorrowedList(self.0.clone()))
//     // }
// }
