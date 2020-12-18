use std::sync::atomic::AtomicPtr;
use std::sync::Arc;

struct ReaderHandler<T> {
    epochs: crate::Epochs,
    inner: Arc<AtomicPtr<T>>,
}
