use core::{any, mem::MaybeUninit, ops};

/// A pointer type that uniquely owns a heap allocation of type T.
#[repr(C)]
pub struct UniquePtr<T> {
    ptr: MaybeUninit<*mut T>,
}

impl<T> UniquePtr<T> {
    /// Returns `true` if the underlying pointer is null.
    pub const fn is_null(&self) -> bool {
        self.as_ptr().is_null()
    }

    /// Returns the underlying pointer as a raw pointer.
    pub const fn as_ptr(&self) -> *const T {
        unsafe { self.ptr.assume_init() }
    }

    /// Returns the underlying pointer as a mutable raw pointer.
    pub const fn as_mut_ptr(&self) -> *mut T {
        unsafe { self.ptr.assume_init() }
    }

    /// Returns `None` if the pointer is null, or else returns a shared reference to
    /// the value wrapped in `Some`. If the value is known to be non-null,
    /// [`as_ref_unchecked`] can be used instead.
    ///
    /// [`as_ref_unchecked`]: #method.as_ref_unchecked
    pub const fn as_ref(&self) -> Option<&T> {
        let ptr = self.as_ptr();
        unsafe { ptr.as_ref() }
    }

    /// Returns a shared reference to the value behind the pointer.
    /// If the pointer may be null, but the value is known to have been initialized, [`as_ref`] must be used instead.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    ///
    /// [`as_ref`]: #method.as_ref
    pub const unsafe fn as_ref_unchecked<'a>(self) -> &'a T {
        // SAFETY: the caller must guarantee that `self` is valid for a reference
        unsafe { &*self.as_ptr() }
    }

    /// Returns `None` if the pointer is null, or else returns a unique reference to
    /// the value wrapped in `Some`. If the value is known to be non-null,
    /// [`as_mut_unchecked`] can be used instead.
    ///
    /// [`as_mut_unchecked`]: #method.as_mut_unchecked
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        let ptr = self.as_mut_ptr();
        unsafe { ptr.as_mut() }
    }

    /// Returns a unique reference to the value behind the pointer.
    /// If the pointer may be null, but the value is known to have been initialized, [`as_mut`] must be used instead.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    ///
    /// [`as_mut`]: #method.as_mut
    pub const unsafe fn as_mut_unchecked<'a>(self) -> &'a mut T {
        // SAFETY: the caller must guarantee that `self` is valid for a reference
        unsafe { &mut *self.as_mut_ptr() }
    }
}

impl<T> ops::Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null UniquePtr<{}>",
                any::type_name::<T>(),
            ),
        }
    }
}

impl<T> ops::DerefMut for UniquePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null UniquePtr<{}>",
                any::type_name::<T>(),
            ),
        }
    }
}
