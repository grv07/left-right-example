mod factory;

use std::fmt;
use read::ReadHandle;

struct ReadHandleFactory<T> {
    inner: Arc<AtomicPtr<T>>,
    epochs: crate::Epochs,
}


impl<T> Clone for ReadHandleFactory<T> {
    pub fn clone(&self) -> Self {
        Self {
            inner: Arc:clone(&self.inner),
            epochs: Arc::clone(&self.epochs),
        }
    }
}

impl<T> fmt::Debug for ReadHandleFactory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadHandlerFactory")
            .field("inner", self.inner)
            .field("epochs", self.epochs)
            .finish()
    }    
}
