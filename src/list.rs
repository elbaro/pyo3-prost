// ## List<T: ToPyObject>

// struct BorrowedList<Owner, Item>(OwningRef<Owner, [Item]>);

// struct BorrowedList<Owner, Borrowed> {
//   owner: Arc<Owner>,
//   borrowed: &'this [Borrowed],
// }

// impl ProxyListTrait for BorrowedList<Owner, Item> {
//   fn len (&self) -> usize{ self.0.len() }
//   fn get<'py>(&self, idx: usize, py: Python<'py>) -> &'py PyAny { self.0[i] }
//   <!-- fn slice(&self, selector: ()) -> Box<dyn ProxyListTrait> {  -->
//     <!-- self.0.clone().map(|slice| slice[selector]) -->
//     <!-- Box::new(BorrowedList(self.0.clone())) -->
//   <!-- } -->
// }

// #[pyclass]
// struct ProxyList(Box<dyn ProxyListTrait>) {

// }

// #[pymethod]
// impl ProxyList {
//   pub fn len(&self) -> usize {
//     self.0.len()
//   }
//   pub fn get(&self) -> &PyAny {
//     self.0.get()
//   }
// }
