use std::cell::Cell;
use std::fmt::{Debug, Formatter, Result};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;

use std::ptr::NonNull;

mod factory;
mod read;

use factory::ReadHandleFactory;

pub struct ReadHandle<T> {
    // maintain the list if all epochs used by all of the readers.
    epochs: crate::Epochs,
    // maintain cuhhent epoch state for reader only.
    epoch: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    // maintainf epoch index
    epoch_i: usize,
    // represent inner data structure.
    pub inner: std::sync::Arc<std::sync::atomic::AtomicPtr<T>>,
    // maintains state if read is not free yet
    enters: std::cell::Cell<usize>,
}

impl<T> Drop for ReadHandle<T> {
    fn drop(&mut self) {
        let e = self.epochs.lock().unwrap().remove(self.epoch_i);
        assert!(std::sync::Arc::ptr_eq(&e, &self.epoch));
        assert_eq!(self.enters.get(), 0);
    }
}

impl<T> Debug for ReadHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ReadHandle")
            .field("inner", &self.inner)
            .field("epoch", &self.epoch)
            .finish()
    }
}

impl<T> Clone for ReadHandle<T> {
    fn clone(&self) -> Self {
        Self::new_with_arc(self.inner.clone(), self.epochs.clone())
    }
}

impl<T> ReadHandle<T> {
    pub fn new(inner: T, epochs: crate::Epochs) -> Self {
        let store = Box::into_raw(Box::new(inner));
        let inner = Arc::new(AtomicPtr::new(store));
        Self::new_with_arc(inner, epochs)
    }

    pub fn new_with_arc(inner: Arc<AtomicPtr<T>>, epochs: crate::Epochs) -> Self {
        let epoch = Arc::new(AtomicUsize::new(0));
        let epoch_i = epochs.lock().unwrap().insert(Arc::clone(&epoch.clone()));

        Self {
            inner,
            epochs,
            enters: Cell::new(0),
            epoch,
            epoch_i,
        }
    }
}

impl<T> ReadHandle<T> {
    pub fn enter(&self) {
        let enters = self.enters.get();
        if enters != 0 {
            self.enters.set(enters + 1);
            return;
        }
        self.epoch.fetch_add(1, Ordering::SeqCst);
        self.enters.set(enters + 1);

        todo!("Implements enter for read")
    }

    pub fn was_dropped(&self) -> bool {
        self.inner.load(Ordering::Relaxed).is_null()
    }

    pub fn raw_handle(&self) -> Option<NonNull<T>> {
        let inner_clone = Arc::clone(&self.inner);
        let inner = inner_clone.load(Ordering::Relaxed);
        NonNull::new(inner)
    }
}
