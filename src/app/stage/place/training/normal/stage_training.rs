use crate::app::StageBase;

#[repr(C)]
pub struct StageTraining {
    pub base: StageBase,
    _738: [u8; 0x8],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTraining>(), 0x740);
        assert_eq!(offset_of!(StageTraining, base), 0x0);
        assert_eq!(offset_of!(StageTraining, _738), 0x738);
    }
}
