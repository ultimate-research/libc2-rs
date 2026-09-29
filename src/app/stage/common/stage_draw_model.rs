use crate::{
    cpp,
    lib::Color4f,
    phx::{Hash40, Vector3f},
};

#[repr(C)]
pub struct StageDrawModel {
    pub _0: Color4f,
    pub _10: Color4f,
    pub _20: Color4f,
    pub _30: Color4f,
    pub _40: f32,
    pub _44: f32,
    _48: [u8; 0x8],
    pub _50: Vector3f,
    pub _60: StageDrawModelSub1,
    pub _f0: u8,
    _f1: [u8; 0x7],
    pub _f8: StageDrawModelSub2,
    pub _108: StageDrawModelSub2,
    pub _118: u8,
}

impl StageDrawModel {
    /// Returns the hashed name of the associated model directory.
    pub const fn model_name(&self) -> Hash40 {
        self._60.dir_name
    }
}

#[repr(C)]
pub struct StageDrawModelSub1 {
    pub dir_handle: u32,
    pub dir_name: Hash40,
    pub _10: u32,
    pub _18: cpp::Vector<()>,
    pub _30: cpp::SharedPtr<()>,
    pub _40: cpp::SharedPtr<()>,
    pub material_motion_controller: cpp::UniquePtr<()>,
    pub _58: cpp::UniquePtr<[u8; 0x60]>,
    _60: [u8; 0x8],
    pub visibility_motion_controller: cpp::UniquePtr<()>,
    _70: [u8; 0x8],
    pub skeleton_motion_controller: cpp::UniquePtr<()>,
    pub _80: bool,
    pub _81: bool,
    pub _82: bool,
    _83: [u8; 0x5],
    pub _88: bool,
}

#[repr(C)]
pub struct StageDrawModelSub2 {
    pub _0: *mut StageDrawModel,
    pub _8: bool,
    pub _9: bool,
    pub _a: bool,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDrawModel>(), 0x120);
        assert_eq!(offset_of!(StageDrawModel, _0), 0x0);
        assert_eq!(offset_of!(StageDrawModel, _10), 0x10);
        assert_eq!(offset_of!(StageDrawModel, _20), 0x20);
        assert_eq!(offset_of!(StageDrawModel, _30), 0x30);
        assert_eq!(offset_of!(StageDrawModel, _40), 0x40);
        assert_eq!(offset_of!(StageDrawModel, _48), 0x48);
        assert_eq!(offset_of!(StageDrawModel, _50), 0x50);
        assert_eq!(offset_of!(StageDrawModel, _60), 0x60);
        assert_eq!(offset_of!(StageDrawModel, _f0), 0xF0);
        assert_eq!(offset_of!(StageDrawModel, _f8), 0xF8);
        assert_eq!(offset_of!(StageDrawModel, _108), 0x108);
        assert_eq!(offset_of!(StageDrawModel, _118), 0x118);

        assert_eq!(size_of::<StageDrawModelSub1>(), 0x90);
        assert_eq!(offset_of!(StageDrawModelSub1, dir_handle), 0x0);
        assert_eq!(offset_of!(StageDrawModelSub1, dir_name), 0x8);
        assert_eq!(offset_of!(StageDrawModelSub1, _10), 0x10);
        assert_eq!(offset_of!(StageDrawModelSub1, _18), 0x18);
        assert_eq!(offset_of!(StageDrawModelSub1, _30), 0x30);
        assert_eq!(offset_of!(StageDrawModelSub1, _40), 0x40);
        assert_eq!(offset_of!(StageDrawModelSub1, material_motion_controller), 0x50);
        assert_eq!(offset_of!(StageDrawModelSub1, _58), 0x58);
        assert_eq!(offset_of!(StageDrawModelSub1, visibility_motion_controller), 0x68);
        assert_eq!(offset_of!(StageDrawModelSub1, skeleton_motion_controller), 0x78);
        assert_eq!(offset_of!(StageDrawModelSub1, _80), 0x80);
        assert_eq!(offset_of!(StageDrawModelSub1, _81), 0x81);
        assert_eq!(offset_of!(StageDrawModelSub1, _82), 0x82);
        assert_eq!(offset_of!(StageDrawModelSub1, _88), 0x88);

        assert_eq!(size_of::<StageDrawModelSub2>(), 0x10);
        assert_eq!(offset_of!(StageDrawModelSub2, _0), 0x0);
        assert_eq!(offset_of!(StageDrawModelSub2, _8), 0x8);
        assert_eq!(offset_of!(StageDrawModelSub2, _9), 0x9);
        assert_eq!(offset_of!(StageDrawModelSub2, _a), 0xA);
    }
}
