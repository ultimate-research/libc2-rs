use core::{ffi::c_void, ops};

#[repr(C)]
pub struct SharedPtr<T> {
    ptr: *mut T,
    cntrl: *mut c_void,
}

impl<T> ops::Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> ops::DerefMut for SharedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
