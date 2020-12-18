extern crate slab;

use std::ptr::NonNull;
use slab::Slab;
use std::sync::Arc;


struct WriterHandler<T> {
    inner: NonNull<T>,
    epochs: Epochs  
}



