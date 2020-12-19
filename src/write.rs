extern crate slab;
use std::sync::Arc;
use crate::read::ReadHandle;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

struct WriteHandle<T> {
    w_handle: NonNull<T>,
    epochs: crate::Epochs,
    r_handle: ReadHandle<T>
}

impl<T> WriteHandle<T> {
    pub fn new(w_handle: NonNull<T>,r_handle: ReadHandle<T>, epochs: crate::Epochs) -> Self {
        Self {
            w_handle: w_handle,
            epochs: epochs,
            r_handle: r_handle 
        }
    }

    pub fn publish(&mut self) {
        let r_handle = unsafe {self.r_handle.inner.swap(self.w_handle.as_mut(), Ordering::Acquire)};
        self.w_handle = NonNull::new(r_handle).unwrap();
    }
}
