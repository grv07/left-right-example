use std::sync::Arc;
use std::sync::Atomic::AtomicPtr;

struct ReaderHandler {
    epochs: crate::Epochs,   
    inner: Arc<AtomicPtr<T>>
}
