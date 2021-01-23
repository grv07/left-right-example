use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct ReadGuard<'rh, T: ?Sized> {
    t: &'rh T,
    epoch: &'rh Arc<AtomicUsize>,
    enters: &'rh Cell<usize>,
}

impl<'rh, T: ?Sized> Drop for ReadGuard<'rh, T> {
    fn drop(&mut self) {
        let enters = self.enters.get() - 1;
        self.enters.set(enters);
        if enters == 0 {
            self.epoch.fetch_add(1, Ordering::Relaxed);
        }
    }
}
