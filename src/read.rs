use std::sync::atomic::{AtomicPtr, AtomicUsize};
use std::sync::Arc;

#[derive(Debug)]
pub struct ReadHandle<T> {
    pub epochs: crate::Epochs,
    pub epoch: Arc<AtomicUsize>,
    pub inner: Arc<AtomicPtr<T>>,
    pub epoch_i: usize,
}

impl<T> Clone for ReadHandle<T> {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);
        let epochs = Arc::clone(&self.epochs);
        Self::new_with_arc(inner, epochs)
    }
}

impl<T> ReadHandle<T> {
    pub fn new(inner: T, epochs: crate::Epochs) -> Self {
        let store = Box::into_raw(Box::new(inner));
        let inner = Arc::new(AtomicPtr::new(store));
        Self::new_with_arc(inner, epochs)
    }

    fn new_with_arc(inner: Arc<AtomicPtr<T>>, epochs: crate::Epochs) -> Self {
        let epoch = Arc::new(AtomicUsize::new(0));
        let epoch_i = epochs.lock().unwrap().insert(Arc::clone(&epoch));
        Self {
            epochs: epochs,
            inner: inner,
            epoch: epoch,
            epoch_i: epoch_i,
        }
    }
}
