use core::slice;

use crate::{app::StageDynamicObject, cpp, phx::Hash40};

#[repr(C)]
pub struct StageLevelData {
    vtable: &'static StageLevelDataVtable,
    _8: usize,
    _10: usize,
    pub dynamic_object_collection: cpp::UniquePtr<StageDynamicObjectCollection>,
    pub instanced_object_collection: cpp::UniquePtr<StageInstancedObjectCollection>,
    _28: usize,
    _30: cpp::Vector<()>,
    _48: cpp::Vector<()>,
    _60: usize,
    _68: usize,
    _70: usize,
}

impl StageLevelData {
    pub fn search_dynamic_object(&self, name: Hash40) -> Option<*mut StageDynamicObject> {
        self.dynamic_object_collection
            .iter()
            .find(|o| o.name_hash == name)
            .map(|o| o.as_mut_ptr())
    }
}

#[repr(C)]
struct StageLevelDataVtable {
    destroy: extern "C" fn(*mut StageLevelData),
    delete: extern "C" fn(*mut StageLevelData),
    init: extern "C" fn(*mut StageLevelData, usize),
    _18: usize,
    _20: usize,
    _28: usize,
    _30: usize,
    _38: usize,
    _40: usize,
    _48: usize,
}

#[repr(C)]
pub struct StageDynamicObjectCollection {
    inner: cpp::Vector<cpp::UniquePtr<StageDynamicObject>>,
}

impl StageDynamicObjectCollection {
    /// Returns an iterator visiting all objects in the underlying collection.
    pub fn iter(&self) -> slice::Iter<'_, cpp::UniquePtr<StageDynamicObject>> {
        self.inner.iter()
    }

    /// Returns an iterator visiting all objects in the underlying collection,
    /// with mutable references to the values.
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, cpp::UniquePtr<StageDynamicObject>> {
        self.inner.iter_mut()
    }
}

#[repr(C)]
pub struct StageInstancedObjectCollection {
    inner: cpp::Vector<cpp::UniquePtr<StageDynamicObject>>,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageLevelData>(), 0x78);
        assert_eq!(offset_of!(StageLevelData, vtable), 0x0);
        assert_eq!(offset_of!(StageLevelData, _8), 0x8);
        assert_eq!(offset_of!(StageLevelData, _10), 0x10);
        assert_eq!(offset_of!(StageLevelData, dynamic_object_collection), 0x18);
        assert_eq!(offset_of!(StageLevelData, instanced_object_collection), 0x20);
        assert_eq!(offset_of!(StageLevelData, _28), 0x28);
        assert_eq!(offset_of!(StageLevelData, _30), 0x30);
        assert_eq!(offset_of!(StageLevelData, _48), 0x48);
        assert_eq!(offset_of!(StageLevelData, _60), 0x60);
        assert_eq!(offset_of!(StageLevelData, _68), 0x68);
        assert_eq!(offset_of!(StageLevelData, _70), 0x70);
    }
}
