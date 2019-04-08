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
        macro_rules! clone {
            ($id: expr; $($i: ident)*) => {
                match $id {
                    [$($i),*] => [$($i.clone()),*]
                }
            }
        }

        ByteMap(clone!(&self.0;
               a0 b0 c0 d0 e0 f0 g0 h0 i0 j0 k0 l0 m0 n0 o0 p0 q0 r0 s0 t0 u0 v0 x0 y0 z0
               a1 b1 c1 d1 e1 f1 g1 h1 i1 j1 k1 l1 m1 n1 o1 p1 q1 r1 s1 t1 u1 v1 x1 y1 z1
               a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2 m2 n2 o2 p2 q2 r2 s2 t2 u2 v2 x2 y2 z2
               a3 b3 c3 d3 e3 f3 g3 h3 i3 j3 k3 l3 m3 n3 o3 p3 q3 r3 s3 t3 u3 v3 x3 y3 z3
               a4 b4 c4 d4 e4 f4 g4 h4 i4 j4 k4 l4 m4 n4 o4 p4 q4 r4 s4 t4 u4 v4 x4 y4 z4
               a5 b5 c5 d5 e5 f5 g5 h5 i5 j5 k5 l5 m5 n5 o5 p5 q5 r5 s5 t5 u5 v5 x5 y5 z5
               a6 b6 c6 d6 e6 f6 g6 h6 i6 j6 k6 l6 m6 n6 o6 p6 q6 r6 s6 t6 u6 v6 x6 y6 z6
               a7 b7 c7 d7 e7 f7 g7 h7 i7 j7 k7 l7 m7 n7 o7 p7 q7 r7 s7 t7 u7 v7 x7 y7 z7
               a9 b9 c9 d9 e9 f9 g9 h9 i9 j9 k9 l9 m9 n9 o9 p9 q9 r9 s9 t9 u9 v9 x9 y9 z9
               a10 b10 c10 d10 e10 f10 g10 h10 i10 j10 k10 l10 m10 n10 o10 p10 q10 r10 s10 t10 u10 v10 x10 y10 z10
               a b c d e f
        ))
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

