use crate::read::ReadHandle;
use crate::write::WriteHandle;
use std::collections::HashMap;

struct ReadFactory<T> {
    inner: Arc<AtomicPtr<T>>,
    epochs: crate::Epochs,
}

impl<T> ReadWriteFactory<T> {
    fn handle(&self) -> ReadHandle {
        let inner = Arc::clone(&self.inner);
        let epochs = Arc::clone(&self.epochs);

        let read = ReadHandle::new_with_arc(inner, epochs);
        read
    }
}
