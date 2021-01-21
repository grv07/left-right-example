use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::read::ReadHandle;
use std::ptr::NonNull;

pub struct WriteHandle<T> { 
    epochs: crate::Epochs,
    pre_epochs: Vec<usize>,
    r_handler: ReadHandle<T>,
    w_handle: NonNull<T>,
    first: bool,
    secound: bool,
}

impl<T> fmt::Debug for WriteHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WriteHandle")
            .field("epochs", &self.epochs)
            .field("pre_epochs", &self.pre_epochs)
            .finish()
    }
}

impl<T> WriteHandle<T> {
    pub fn new(epochs: crate::Epochs, r_handler: ReadHandle<T>, w_handle: T) -> Self {
        let w_handle = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(w_handle)))};
        Self {
            epochs,
            r_handler,
            w_handle,
            pre_epochs: Vec::new(),
            first: true,
            secound: true,
            swap_index: usize,
        }
    }

    fn wait() {}
    
    fn publish(&mut self) -> &mut Self { 
        let epochs = Arc::clone(&self.epochs);
        let mut epochs = epochs.lock().unwrap(); 

        let cap = epochs.capacity();
        let pre_epoch = Vec::<usize>::with_capacity(cap);
        Self::wait(); 
        
        if !self.first {
                    
        }
        else{
        
        }
        self.first = false;

        let new_r_handle = self.r_handler.inner.swap(self.w_handle.as_ptr(), Ordering::Relaxed);
        self.w_handle = unsafe {NonNull::new_unchecked(new_r_handle)};
        for (ri, epoch) in epochs.iter() {
            self.pre_epochs[ri] = epoch.load(Ordering::Relaxed);
        }
       self 
    }
}
