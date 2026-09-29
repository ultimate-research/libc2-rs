use crate::{
    app::{StageAttr, StageID},
    phx::Hash40,
};

mod ex;
mod random;

pub use ex::*;
pub use random::*;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GlobalStageParameter {
    stage_id: StageID,
    stage_resource_path: Hash40,
    pub is_gimmick: bool,
    _11: [u8; 0x6],
    _17: u8,
    pub floor_place_id: Hash40,
    pub attr: StageAttr,
    pub ui_bgm_id: Hash40,
    _30: u8,
    _31: [u8; 0xF],
    pub setting: StageSetting,
}

impl GlobalStageParameter {
    /// Returns a reference to the working stage identifier.
    pub const fn stage_id(&self) -> &StageID {
        &self.stage_id
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct StageSetting {
    pub ex: StageExSetting,
    _30: [u8; 0x10],
    pub random: StageRandomSetting,
    _68: [u8; 0x4],
    pub _6c: i32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<GlobalStageParameter>(), 0xB0);
        assert_eq!(offset_of!(GlobalStageParameter, stage_id), 0x0);
        assert_eq!(offset_of!(GlobalStageParameter, stage_resource_path), 0x8);
        assert_eq!(offset_of!(GlobalStageParameter, is_gimmick), 0x10);
        assert_eq!(offset_of!(GlobalStageParameter, _17), 0x17);
        assert_eq!(offset_of!(GlobalStageParameter, floor_place_id), 0x18);
        assert_eq!(offset_of!(GlobalStageParameter, attr), 0x20);
        assert_eq!(offset_of!(GlobalStageParameter, ui_bgm_id), 0x28);
        assert_eq!(offset_of!(GlobalStageParameter, _30), 0x30);
        assert_eq!(offset_of!(GlobalStageParameter, setting), 0x40);

        assert_eq!(size_of::<StageSetting>(), 0x70);
        assert_eq!(offset_of!(StageSetting, ex), 0x0);
        assert_eq!(offset_of!(StageSetting, random), 0x40);
        assert_eq!(offset_of!(StageSetting, _6c), 0x6C);
    }
}
