#[repr(C)]
pub struct StageBossStageMarx {
    _0: [u8; 0x770],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageMarx>(), 0x770);
        assert_eq!(offset_of!(StageBossStageMarx, _0), 0x0);
    }
}
