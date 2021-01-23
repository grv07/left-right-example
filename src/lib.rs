use slab::Slab;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

mod read;
pub use crate::read::ReadHandle;

mod write;

pub type Epochs = Arc<Mutex<Slab<Arc<AtomicUsize>>>>;

trait Absorb<O> {
    fn absorb_first(&mut self, op: &mut O, other: &Self);

    fn absorb_second(&mut self, op: O, other: &Self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
