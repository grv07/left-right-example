extern crate slab;
use crate::read::ReadHandle;
use slab::Slab;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct WriteHandle<T> {
    w_handle: NonNull<T>,
    epochs: crate::Epochs,
    r_handle: ReadHandle<T>,
}

impl<T> WriteHandle<T> {
    pub fn new(w_handle: NonNull<T>, r_handle: ReadHandle<T>, epochs: crate::Epochs) -> Self {
        Self {
            w_handle: w_handle,
            epochs: epochs,
            r_handle: r_handle,
        }
    }

    pub fn publish(&mut self) {
        let r_handle = unsafe {
            self.r_handle
                .inner
                .swap(self.w_handle.as_mut(), Ordering::Acquire)
        };
        self.w_handle = NonNull::new(r_handle).unwrap();
    }
}

#[test]
fn test_crate_write_handle() {
    let mut m1 = Box::new(HashMap::new());
    m1.insert("w", "1");
    let mut m2 = HashMap::new();
    m2.insert("r", "1");

    let w_handle = NonNull::new(Box::into_raw(m1)).unwrap();
    let mut slab = Slab::new();
    slab.insert(Arc::new(AtomicUsize::new(0)));
    let epochs = Arc::new(Mutex::new(slab));

    let r_handle = ReadHandle::new(m2, Arc::clone(&epochs));

    let mut wh = WriteHandle::new(w_handle, r_handle, epochs);
    wh.publish();

    let w_map = unsafe { &wh.w_handle.as_ref() };
    assert_eq!(w_map.contains_key("r"), true);

    let r_map = wh.r_handle.inner.load(Ordering::Acquire);
    let r_map = NonNull::new(r_map).unwrap();
    let r_map = unsafe { r_map.as_ref() };

    assert_eq!(r_map.contains_key("w"), true);
}
