use num::PrimInt;

use super::Fnv0;

// #[cfg(test)]
// mod tests;

/// The wrapper type for the result of the FNV-1 hash function.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Fnv1 {
    inner: u32,
}

impl Fnv1 {
    /// The initial value of the hash.
    const OFFSET_BASIS: u32 = Fnv0::new("chongo <Landon Curt Noll> /\\../\\").into_raw();

    /// The arbitrary prime number for hash dispersion.
    ///
    /// The defined value is not standard, favoring what was used in the
    /// implementation found in Super Smash Bros. for Nintendo 3DS / Wii U and
    /// Super Smash Bros. Ultimate.
    const PRIME: u32 = 137;

    /// Creates a new `Fnv1`.
    pub const fn new() -> Self {
        Self {
            inner: Self::OFFSET_BASIS,
        }
    }

    /// Computes a new `Fnv1` from an integer slice.
    fn from<T: PrimInt>(buf: &[T]) -> Self {
        let mut hash = Self::OFFSET_BASIS;

        for value in buf.iter().map_while(|v| v.to_u32()) {
            hash = u32::wrapping_mul(hash, Self::PRIME) ^ value;
        }

        Self { inner: hash }
    }
}

impl Default for Fnv1 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[u8]> for Fnv1 {
    fn from(buf: &[u8]) -> Self {
        Self::from::<u8>(buf)
    }
}

impl From<&[u16]> for Fnv1 {
    fn from(buf: &[u16]) -> Self {
        Self::from::<u16>(buf)
    }
}
