use std::fmt;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

use AllBytesIter;

#[derive(Copy)]
pub struct ByteMap<S>([S; 256]);

impl<S: fmt::Debug> fmt::Debug for ByteMap<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("ByteMap")
            .field(&&self.0[..])
            .finish()
    }
}

impl<S: Clone> Clone for ByteMap<S> {
    fn clone(&self) -> Self {
        unsafe {
            let mut out = mem::uninitialized::<mem::ManuallyDrop<ByteMap<S>>>();
            for i in AllBytesIter::new() {
                ptr::write(&mut out[i], self[i].clone());
            }
            mem::ManuallyDrop::into_inner(out)
        }
    }
}

impl<S: Copy> ByteMap<S> {
    pub fn new(s: S) -> Self {
        ByteMap([s; 256])
    }
}

impl<S> ByteMap<S> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_inner(self) -> [S; 256] {
        self.0
    }
}

impl<S> Index<u8> for ByteMap<S> {
    type Output = S;

    #[inline(always)]
    fn index(&self, i: u8) -> &S {
        // SAFETY: This is safe because all dense transitions have
        // exactly 256 elements, so all u8 values are valid indices.
        unsafe { self.0.get_unchecked(i as usize) }
    }
}

impl<S> IndexMut<u8> for ByteMap<S> {
    #[inline(always)]
    fn index_mut(&mut self, i: u8) -> &mut S {
        // SAFETY: This is safe because all dense transitions have
        // exactly 256 elements, so all u8 values are valid indices.
        unsafe { self.0.get_unchecked_mut(i as usize) }
    }
}

