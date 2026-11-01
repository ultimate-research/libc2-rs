use crate::{app::StageBaseStorage, cpp};

/// The stage where the results take place.
#[repr(C, align(16))]
pub struct StageResultStage {
    pub base: StageBaseStorage,
    pub _738: cpp::UniquePtr<()>,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageResultStage>(), 0x740);
        assert_eq!(offset_of!(StageResultStage, base), 0x0);
        assert_eq!(offset_of!(StageResultStage, _738), 0x738);
    }
}
