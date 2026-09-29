use core::{any, fmt, ops, ptr, slice};

/// A contiguous growable array type.
#[repr(C)]
pub struct Vector<T> {
    begin: *mut T,
    end: *mut T,
    capacity: *mut T,
}

impl<T> Vector<T> {
    /// Constructs a new, empty `Vector<T>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    pub const fn new() -> Self {
        Self {
            begin: ptr::null_mut(),
            end: ptr::null_mut(),
            capacity: ptr::null_mut(),
        }
    }

    /// Returns the number of elements in the vector, also referred to as its ‘length’.
    pub const fn len(&self) -> usize {
        unsafe { self.end.offset_from_unsigned(self.begin) }
    }

    /// Returns the total number of elements the vector can hold without reallocating.
    pub const fn capacity(&self) -> usize {
        unsafe { self.capacity.offset_from_unsigned(self.begin) }
    }

    /// Returns `true` if the vector contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extracts a slice containing the entire vector.
    pub const fn as_slice(&self) -> Option<&[T]> {
        if self.begin.is_null() {
            return None;
        }

        unsafe { Some(slice::from_raw_parts(self.begin, self.len())) }
    }

    /// Extracts a slice containing the entire vector.
    pub const fn as_mut_slice(&mut self) -> Option<&mut [T]> {
        if self.begin.is_null() {
            return None;
        }

        unsafe { Some(slice::from_raw_parts_mut(self.begin, self.len())) }
    }
}

impl<T> AsRef<Vector<T>> for Vector<T> {
    fn as_ref(&self) -> &Vector<T> {
        self
    }
}

impl<T> AsMut<Vector<T>> for Vector<T> {
    fn as_mut(&mut self) -> &mut Vector<T> {
        self
    }
}

impl<T> AsRef<[T]> for Vector<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T> AsMut<[T]> for Vector<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ops::Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self.as_slice() {
            Some(target) => target,
            None => panic!("called deref on a null Vector<{}>", any::type_name::<T>()),
        }
    }
}

impl<T> ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut_slice() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null Vector<{}>",
                any::type_name::<T>(),
            ),
        }
    }
}

impl<T, I: slice::SliceIndex<[T]>> ops::Index<I> for Vector<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(&**self, index)
    }
}

impl<T, I: slice::SliceIndex<[T]>> ops::IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(&mut **self, index)
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vector<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
