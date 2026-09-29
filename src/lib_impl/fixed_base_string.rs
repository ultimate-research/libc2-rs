use core::{
    fmt,
    str::{FromStr, Utf8Error},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use thiserror::Error;

mod fnv0;
mod fnv1;

use fnv0::Fnv0;
use fnv1::Fnv1;

/// A nul-terminated string with a fixed capacity of 64 bytes.
pub type FixedBaseString64 = FixedBaseString<{ 63 + 1 }>;

/// A nul-terminated string with a fixed capacity.
#[repr(C)]
pub struct FixedBaseString<const N: usize> {
    /// The precomputed FNV-1 hash derived from the character buffer.
    hash: Fnv1,

    /// The length of the string, excluding the nul byte.
    len: u32,

    /// The fixed-capacity, nul-terminated string.
    buf: [u8; N],
}

impl<const N: usize> FixedBaseString<N> {
    /// The number of characters the buffer can hold, including the nul byte.
    pub const CAPACITY: usize = N;

    /// The number of characters the buffer can hold, excluding the nul byte.
    pub const CAPACITY_WITHOUT_NUL: usize = N - 1;

    /// Creates a new empty `FixedBaseString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            hash: Fnv1::new(),
            len: 0,
            buf: [0; N],
        }
    }

    /// Returns the length of the contained string.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::try_from("curve1").unwrap();
    /// assert_eq!(s.len(), 6);
    /// ```
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the contained string has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::new();
    /// assert!(s.is_empty());
    ///
    /// let s = FixedBaseString::<64>::try_from("curve2").unwrap();
    /// assert!(!s.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the contents of this `FixedBaseString` as a byte slice.
    ///
    /// The returned slice does **not** contain the trailing nul
    /// terminator, and it is guaranteed to not have any interior nul bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::try_from("curve3").unwrap();
    /// assert_eq!(s.as_bytes(), b"curve3");
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len()]
    }

    /// Converts the underlying buffer to a string slice if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::try_from("curve4").unwrap();
    /// assert_eq!(s.to_str().unwrap(), "curve4");
    /// ```
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts the underlying buffer to a [`String`] if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use libc2::lib::FixedBaseString;
    ///
    /// let s = FixedBaseString::<64>::try_from("curve5").unwrap();
    /// assert_eq!(s.to_string().unwrap(), "curve5".to_string());
    /// ```
    pub fn to_string(&self) -> Result<String, Utf8Error> {
        self.to_str().map(|s| s.to_string())
    }
}

impl<const N: usize> Default for FixedBaseString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for FixedBaseString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().unwrap())
    }
}

impl<const N: usize> FromStr for FixedBaseString<N> {
    type Err = ParseFixedBaseStringError<N>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > Self::CAPACITY_WITHOUT_NUL {
            return Err(Self::Err::BufferOverflow);
        }

        let mut buf = [0; N];
        let bytes = s.as_bytes();

        for (index, byte) in bytes.iter().copied().enumerate() {
            buf[index] = byte;
        }

        Ok(Self {
            hash: Fnv1::from(bytes),
            len: bytes.len() as u32,
            buf,
        })
    }
}

impl<const N: usize> TryFrom<&String> for FixedBaseString<N> {
    type Error = ParseFixedBaseStringError<N>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> TryFrom<&str> for FixedBaseString<N> {
    type Error = ParseFixedBaseStringError<N>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> TryFrom<String> for FixedBaseString<N> {
    type Error = ParseFixedBaseStringError<N>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl<const N: usize> PartialEq for FixedBaseString<N> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for FixedBaseString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self.to_str().map_err(serde::ser::Error::custom)?;

        serializer.serialize_str(string)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for FixedBaseString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

/// The error type used when converting a string into a [`FixedBaseString`].
#[derive(Debug, PartialEq, Error)]
pub enum ParseFixedBaseStringError<const N: usize> {
    /// The nul-terminated string exceeds the buffer's capacity.
    #[error("nul-terminated string exceeds buffer capacity of {} bytes", N)]
    BufferOverflow,
}
