use num_traits::{CheckedAdd, One, Unsigned};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id<T: Unsigned>(T);

pub type Id8 = Id<u8>;
pub type Id16 = Id<u16>;
pub type Id32 = Id<u32>;
pub type Id64 = Id<u64>;

impl<T: Unsigned + Copy + CheckedAdd + One> Id<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    pub const fn get(&self) -> T {
        self.0
    }

    pub fn increment(&self) -> Option<Self> {
        self.0.checked_add(&T::one()).map(Self)
    }
}
