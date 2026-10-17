use crate::app::StageBaseStorage;

/// The Training stage.
#[repr(C, align(16))]
pub struct StageTraining {
    pub base: StageBaseStorage,
    pub _731: bool,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTraining>(), 0x740);
        assert_eq!(offset_of!(StageTraining, base), 0x0);
        assert_eq!(offset_of!(StageTraining, _731), 0x731);
    }
}
