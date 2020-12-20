use crate::read::ReadHandle;
use crate::write::WriteHandle;

use slab::Slab;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

struct LeftRight<T> {
    inner: Arc<AtomicPtr<T>>,
    epochs: crate::Epochs,
}

impl<T> LeftRight<T> {
    fn new(&self) -> (ReadHandle<T>, WriteHandle<T>) {
        let inner_r = Arc::clone(&self.inner);

        let inner_ptr = self.inner.load(Ordering::Relaxed);
        let inner_w = NonNull::new(inner_ptr).unwrap();

        let epochs_r = Arc::clone(&self.epochs);
        let epochs_w = Arc::clone(&self.epochs);

        let read = ReadHandle::new_with_arc(inner_r, epochs_r);
        let write = WriteHandle::new(inner_w, read.clone(), epochs_w);

        (read, write)
    }
}

#[test]
fn test_left_right() {
    let mut data = Box::new(HashMap::new());
    data.insert("data", "data");
    let epochs = Arc::new(Mutex::new(Slab::new()));
    let inner = Arc::new(AtomicPtr::new(Box::into_raw(data)));
    let left_right = LeftRight {
        inner: inner,
        epochs: epochs,
    };
    let (r, w) = left_right.new();
}
