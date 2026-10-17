use crate::{app::StageBaseStorage, cpp};

use super::StageData;

/// The Peach's Castle stage.
#[repr(C, align(16))]
pub struct StageMarioCastle64 {
    pub base: StageBaseStorage,
    pub stage_data: StageData,
    pub bumper: cpp::UniquePtr<()>,
    pub floor: cpp::UniquePtr<()>,
    pub triangle_l: cpp::UniquePtr<()>,
    pub triangle_r: cpp::UniquePtr<()>,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioCastle64>(), 0x810);
        assert_eq!(offset_of!(StageMarioCastle64, base), 0x0);
        assert_eq!(offset_of!(StageMarioCastle64, stage_data), 0x738);
        assert_eq!(offset_of!(StageMarioCastle64, bumper), 0x7E8);
        assert_eq!(offset_of!(StageMarioCastle64, floor), 0x7F0);
        assert_eq!(offset_of!(StageMarioCastle64, triangle_l), 0x7F8);
        assert_eq!(offset_of!(StageMarioCastle64, triangle_r), 0x800);
    }
}
