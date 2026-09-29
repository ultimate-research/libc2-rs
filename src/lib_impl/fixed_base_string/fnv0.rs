/// The wrapper type for the result of the FNV-0 hash function.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Fnv0 {
    inner: u32,
}

impl Fnv0 {
    /// The selected prime number for hash dispersion.
    const PRIME: u32 = 2u32.pow(24) + 2u32.pow(8) + 0x93;

    /// Creates a new `Fnv0`.
    pub const fn new(s: &str) -> Self {
        let mut hash = 0u32;
        let bytes = s.as_bytes();
        let mut index = 0;

        while index < bytes.len() {
            hash = u32::wrapping_mul(hash, Self::PRIME) ^ bytes[index] as u32;
            index += 1;
        }

        Self { inner: hash }
    }

    /// Consumes the `Fnv0`, returning the contained value.
    pub const fn into_raw(self) -> u32 {
        self.inner
    }
}
