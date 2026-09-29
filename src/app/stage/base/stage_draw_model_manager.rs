use crate::{
    app::{GlobalStageParameter, StageBase, StageDrawModel},
    cpp,
};

#[cfg(feature = "impl")]
use crate::phx::Hash40;

#[repr(C)]
pub struct StageDrawModelManager {
    vtable: &'static StageDrawModelManagerVtable,
    stage_base: *const StageBase,
    stage_parameter: GlobalStageParameter,
    _c0: i32,
    _c4: u8,
    _c5: u8,
    _c6: u8,
    _c7: u8,
    _c8: u8,
    pub draw_model_collection: cpp::Vector<StageDrawModelAccessor>,
    pub _e8: cpp::Vector<StageDrawModelAccessor>,
    _100: cpp::Vector<[u8; 0x10]>,
    _118: usize,
}

#[cfg(feature = "impl")]
impl StageDrawModelManager {
    pub fn search_model(&self, name: Hash40) -> Option<&StageDrawModelAccessor> {
        for accessor in &self.draw_model_collection {
            if accessor.category >= StageDrawModelCategory::PTrainerFloor {
                continue;
            }

            let model_name = accessor.draw_model.model_name();

            if model_name != name {
                continue;
            }

            return Some(accessor);
        }

        None
    }

    pub fn instance_model(&mut self, accessor: &StageDrawModelAccessor) -> *mut StageDrawModel {
        unsafe {
            #[skyline::from_offset(0x262FB80)]
            fn inner(
                this: *mut StageDrawModelManager,
                accessor: *const StageDrawModelAccessor,
            ) -> *mut StageDrawModel;

            inner(self, accessor)
        }
    }
}

#[repr(C)]
struct StageDrawModelManagerVtable {
    destroy: extern "C" fn(*mut StageDrawModelManager),
    delete: extern "C" fn(*mut StageDrawModelManager),
    init: extern "C" fn(*mut StageDrawModelManager, usize),
    _18: extern "C" fn(*mut StageDrawModelManager, bool),
    _20: extern "C" fn(*mut StageDrawModelManager, bool),
    _28: extern "C" fn(*mut StageDrawModelManager, usize),
}

#[repr(C)]
pub struct StageDrawModelAccessor {
    pub draw_model: cpp::UniquePtr<StageDrawModel>,
    pub category: StageDrawModelCategory,
    pub is_instance: bool,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub enum StageDrawModelCategory {
    Normal,
    Boss,
    PTrainerFloor,
    KumiteItemBase,
    SpiritsFloor,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDrawModelManager>(), 0x120);
        assert_eq!(offset_of!(StageDrawModelManager, vtable), 0x0);
        assert_eq!(offset_of!(StageDrawModelManager, stage_base), 0x8);
        assert_eq!(offset_of!(StageDrawModelManager, stage_parameter), 0x10);
        assert_eq!(offset_of!(StageDrawModelManager, _c0), 0xC0);
        assert_eq!(offset_of!(StageDrawModelManager, _c4), 0xC4);
        assert_eq!(offset_of!(StageDrawModelManager, _c5), 0xC5);
        assert_eq!(offset_of!(StageDrawModelManager, _c6), 0xC6);
        assert_eq!(offset_of!(StageDrawModelManager, _c7), 0xC7);
        assert_eq!(offset_of!(StageDrawModelManager, _c8), 0xC8);
        assert_eq!(offset_of!(StageDrawModelManager, draw_model_collection), 0xD0);
        assert_eq!(offset_of!(StageDrawModelManager, _e8), 0xE8);
        assert_eq!(offset_of!(StageDrawModelManager, _100), 0x100);
        assert_eq!(offset_of!(StageDrawModelManager, _118), 0x118);
    }
}
