use crate::read::ReadHandle;
use std::fmt;
use std::sync::atomic::AtomicPtr;
use std::sync::Arc;

pub struct ReadHandleFactory<T> {
    inner: Arc<AtomicPtr<T>>,
    epochs: crate::Epochs,
}

impl<T> Clone for ReadHandleFactory<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            epochs: Arc::clone(&self.epochs),
        }
    }
}

impl<T> fmt::Debug for ReadHandleFactory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadHandleFactory")
            .field("inner", &self.inner)
            .field("epochs", &self.epochs)
            .finish()
    }
}

impl<T> ReadHandleFactory<T> {
    pub fn factory(&self) -> ReadHandle<T> {
        ReadHandle::new_with_arc(Arc::clone(&self.inner), Arc::clone(&self.epochs))
    }
}
