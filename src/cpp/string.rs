use core::{ffi::c_char, fmt, ops, slice, str::Utf8Error};

/// A growable 8-bit character string.
#[repr(C, align(8))]
pub struct String {
    inner: [u8; 0x18],
}

impl String {
    /// Creates a new empty `String`.
    pub const fn new() -> Self {
        Self { inner: [0; _] }
    }

    /// Returns the length of this `String`, in bytes, not [`char`]s or
    /// graphemes. In other words, it might not be what a human considers the
    /// length of the string.
    pub const fn len(&self) -> usize {
        if self.is_long() {
            // SAFETY: A pointer to our own array is always valid as long as we
            // do not offset it beyond the size of the `String`.
            unsafe { *self.inner.as_ptr().add(0x8).cast::<usize>() }
        } else {
            (self.inner[0] >> 1) as usize
        }
    }

    /// Returns the number of bytes this `String` can hold without reallocating.
    pub const fn capacity(&self) -> usize {
        if self.is_long() {
            // SAFETY: A pointer to our own array is always valid.
            unsafe { *self.inner.as_ptr().cast::<usize>() & !0x1 }
        } else {
            size_of::<Self>() - 1
        }
    }

    /// Returns `true` if this `String` has a length of zero.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts this `String` to a raw pointer.
    pub const fn as_ptr(&self) -> *const c_char {
        // SAFETY: A pointer to our own array is always valid as long as we
        // do not offset it beyond the size of the `String`.
        unsafe {
            if self.is_long() {
                *self.inner.as_ptr().add(0x10).cast::<*const c_char>()
            } else {
                self.inner.as_ptr().add(0x1).cast::<c_char>()
            }
        }
    }

    /// Converts this `String` to a mutable raw pointer.
    pub const fn as_mut_ptr(&mut self) -> *mut c_char {
        // SAFETY: A pointer to our own array is always valid as long as we
        // do not offset it beyond the size of the `String`.
        unsafe {
            if self.is_long() {
                *self.inner.as_mut_ptr().add(0x10).cast::<*mut c_char>()
            } else {
                self.inner.as_mut_ptr().add(0x1).cast::<c_char>()
            }
        }
    }

    /// Returns a byte slice of this `String`'s contents.
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: Even on a zero-initialized `String`, the raw pointer will
        // still point to valid character data due to short string optimization.
        unsafe { slice::from_raw_parts(self.as_ptr().cast::<u8>(), self.len()) }
    }

    /// Returns a mutable byte slice of this `String`'s contents.
    pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: Even on a zero-initialized `String`, the raw pointer will
        // still point to valid character data due to short string optimization.
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().cast::<u8>(), self.len()) }
    }

    /// Yields a string slice if the `String` contains valid UTF-8.
    pub const fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts this `String` into a string slice.
    pub const unsafe fn as_str_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.as_bytes())
    }

    /// Yields a mutable string slice if the `String` contains valid UTF-8.
    pub const fn to_mut_str(&mut self) -> Result<&mut str, Utf8Error> {
        str::from_utf8_mut(self.as_bytes_mut())
    }

    /// Converts this `String` into a mutable string slice.
    pub const unsafe fn as_mut_str_unchecked(&self) -> &str {
        str::from_utf8_unchecked_mut(self.as_bytes_mut())
    }

    /// Returns `true` if the string is too long to avoid memory allocation.
    const fn is_long(&self) -> bool {
        self.inner[0] & 1 != 0
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str().expect("String should contain valid UTF-8")
    }
}

impl ops::DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.to_mut_str()
            .expect("String should contain valid UTF-8")
    }
}
