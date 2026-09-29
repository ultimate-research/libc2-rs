use crate::phx::{Hash40, Vector3f};

#[repr(C)]
pub struct StageGroundCrackParam {
    pub _0: u32, // TODO: Checked in Wii Fit Studio.
    pub crack_type: StageGroundCrackType,
    pub crack_id: Hash40,
    pub crack_rotate: Vector3f,
    pub crack_scale: Vector3f,
    pub crack_offset: Vector3f,
}

#[repr(C)]
pub enum StageGroundCrackType {
    Decal,
    Effect,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageGroundCrackParam>(), 0x40);
        assert_eq!(offset_of!(StageGroundCrackParam, _0), 0x0);
        assert_eq!(offset_of!(StageGroundCrackParam, crack_type), 0x4);
        assert_eq!(offset_of!(StageGroundCrackParam, crack_id), 0x8);
        assert_eq!(offset_of!(StageGroundCrackParam, crack_rotate), 0x10);
        assert_eq!(offset_of!(StageGroundCrackParam, crack_scale), 0x20);
        assert_eq!(offset_of!(StageGroundCrackParam, crack_offset), 0x30);
    }
}
