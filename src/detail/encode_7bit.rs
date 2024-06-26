use crate::detail::Truncate;
use ux::*;

pub trait Encode7Bit<const N: usize>:
    Sized
    + core::default::Default
    + core::convert::From<u8>
    + core::convert::From<u7>
    + core::ops::BitOrAssign<Self>
    + core::ops::BitAnd<Self, Output = Self>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shr<usize, Output = Self>
    + Truncate
    + core::marker::Copy
where
    u7: core::convert::TryFrom<Self>,
    <u7 as core::convert::TryFrom<Self>>::Error: core::fmt::Debug,
{
    fn from_u7s<T>(u7s: &[T]) -> Self
    where
        Self: core::convert::From<T>,
        T: Copy,
    {
        assert_eq!(u7s.len(), N);
        let mut ret: Self = Default::default();
        for (i, v) in u7s.iter().enumerate() {
            ret |= (Self::from(*v) & Self::from(0b0111_1111_u8)) << (7_usize * i);
        }
        ret
    }

    fn to_u7s(&self, data: &mut [u7]) {
        assert_eq!(data.len(), N);
        for (i, v) in data.iter_mut().enumerate() {
            *v = (*self >> (i * 7_usize)).truncate()
        }
    }
}

impl Encode7Bit<4> for u28 {}
impl Encode7Bit<3> for u21 {}
impl Encode7Bit<2> for u14 {}
