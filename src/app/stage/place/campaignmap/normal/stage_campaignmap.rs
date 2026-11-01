use crate::app::StageBaseStorage;

/// The ad hoc Adventure map stage.
#[repr(C, align(16))]
pub struct StageCampaignMap {
    pub base: StageBaseStorage,
    pub _734: u32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageCampaignMap>(), 0x740);
        assert_eq!(offset_of!(StageCampaignMap, base), 0x0);
        assert_eq!(offset_of!(StageCampaignMap, _734), 0x734);
    }
}
