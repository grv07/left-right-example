use slab::Slab;
use std::sync::Arc;

mod reader;
mod writer;

pub type Epochs = Arc<Slab<usize>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
