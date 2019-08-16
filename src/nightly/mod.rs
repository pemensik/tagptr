mod atomic;
mod raw;

use core::marker::PhantomData;
use core::mem;
use core::sync::atomic::AtomicUsize;
use core::ptr::NonNull;

////////////////////////////////////////////////////////////////////////////////////////////////////
// AtomicMarkedPtr
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct AtomicMarkedPtr<T, const N: usize> {
    ptr: AtomicUsize,
    _marker: PhantomData<*mut T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MarkedPtr
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Hash)]
pub struct MarkedPtr<T, const N: usize> {
    ptr: *mut T,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MarkedNonNull
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MarkedNonNull<T, const N: usize> {
    ptr: NonNull<T>,
}