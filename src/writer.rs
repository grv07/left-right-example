extern crate slab;

use std::ptr::NonNull;

struct WriterHandler<T> {
    inner: NonNull<T>,
    epochs: crate::Epochs,
}
