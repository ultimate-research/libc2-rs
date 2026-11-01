use crate::app::StageBaseStorage;

/// The stage where Home-Run Contest takes place.
#[repr(C, align(16))]
pub struct StageHomerunContest {
    pub base: StageBaseStorage,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageHomerunContest>(), 0x740);
        assert_eq!(offset_of!(StageHomerunContest, base), 0x0);
    }
}
