use crate::app::StageBaseStorage;

use super::StageData;

/// The Kongo Jungle stage.
#[repr(C)]
pub struct StageDKJungle {
    pub base: StageBaseStorage,
    pub stage_data: StageData,
    _818: [u8; 0x98],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDKJungle>(), 0x8B0);
        assert_eq!(offset_of!(StageDKJungle, base), 0x0);
        assert_eq!(offset_of!(StageDKJungle, stage_data), 0x738);
    }
}
