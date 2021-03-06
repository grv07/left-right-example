use crate::read::ReadHandle;
use slab::Slab;
use std::collections::VecDeque;
use std::fmt;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, MutexGuard};

use super::Absorb;

pub struct WriteHandle<T, O> {
    epochs: crate::Epochs,
    pre_epochs: Vec<usize>,
    oplog: VecDeque<O>,
    r_handler: ReadHandle<T>,
    w_handle: NonNull<T>,
    first: bool,
    secound: bool,
    swap_index: usize,
}

impl<T, O> fmt::Debug for WriteHandle<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WriteHandle")
            .field("epochs", &self.epochs)
            .field("pre_epochs", &self.pre_epochs)
            .finish()
    }
}

impl<T, O> WriteHandle<T, O>
where
    T: Absorb<O>,
{
    pub fn new(epochs: crate::Epochs, r_handler: ReadHandle<T>, w_handle: T) -> Self {
        let w_handle = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(w_handle))) };
        Self {
            epochs,
            r_handler,
            w_handle,
            oplog: VecDeque::new(),
            pre_epochs: Vec::new(),
            first: true,
            secound: true,
            swap_index: 0,
        }
    }

    fn wait(&mut self, epochs: &MutexGuard<'_, Slab<Arc<AtomicUsize>>>) {
        let mut istart = 0;
        let mut iter = 0;

        self.pre_epochs.resize(epochs.capacity(), 0);

        'retry: loop {
            for (i, (k, epoch)) in epochs.iter().skip(istart).enumerate() {
                let epoch = epoch.clone().load(Ordering::Relaxed);
                if epoch % 2usize == 0 {
                    continue;
                }
                if epoch != self.pre_epochs[k] {
                    continue;
                } else {
                    istart = i;
                    if iter != 20 {
                        iter += 1;
                        continue;
                    } else {
                        std::thread::yield_now();
                    }
                    continue 'retry;
                }
            }
            break;
        }
    }

    fn publish(&mut self) -> &mut Self {
        let epochs = Arc::clone(&self.epochs);
        let mut epochs = epochs.lock().unwrap();

        let cap = epochs.capacity();
        let pre_epoch = Vec::<usize>::with_capacity(cap);
        self.wait(&mut epochs);

        if !self.first {
            let raw_read_ds = self.r_handler.raw_handle().unwrap();
            let raw_read_ds = unsafe { raw_read_ds.as_ref() };
            let raw_write_ds = unsafe { self.w_handle.as_mut() };
            if self.secound {}
            if self.swap_index != 0 {
                for op in self.oplog.drain(..self.swap_index) {
                    T::absorb_second(raw_write_ds, op, raw_read_ds);
                }
            }
            for op in self.oplog.iter_mut() {
                T::absorb_first(raw_write_ds, op, raw_read_ds);
            }
            self.swap_index = self.oplog.len();
        } else {
        }
        self.first = false;

        let new_r_handle = self
            .r_handler
            .inner
            .swap(self.w_handle.as_ptr(), Ordering::Relaxed);
        self.w_handle = unsafe { NonNull::new_unchecked(new_r_handle) };
        for (ri, epoch) in epochs.iter() {
            self.pre_epochs[ri] = epoch.load(Ordering::Relaxed);
        }
        self
    }
}
