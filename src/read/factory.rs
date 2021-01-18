use read::ReadHandle;

struct ReadHandleFactory<T> {
    inner: Arc<AtomicPtr<T>>,
    epochs: crate::Epochs,
}

