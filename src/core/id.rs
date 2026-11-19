use std::fmt::{Display, UpperHex};

use num_traits::{CheckedAdd, One, ToBytes, Unsigned};

use crate::file::codec::ser::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id<T: Unsigned>(T);

pub type Id8 = Id<u8>;
pub type Id16 = Id<u16>;
pub type Id32 = Id<u32>;
pub type Id64 = Id<u64>;

impl<T: Unsigned + Copy + CheckedAdd + One> Id<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn into_inner(&self) -> T {
        self.0
    }

    pub fn increment(&self) -> Option<Self> {
        self.0.checked_add(&T::one()).map(Self)
    }
}

impl<T: Unsigned + Display> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Unsigned + UpperHex> UpperHex for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl<T> Serialize for Id<T>
where
    T: Unsigned + ToBytes,
    <T as ToBytes>::Bytes: AsRef<[u8]>,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.0.to_le_bytes().as_ref());
    }
}
