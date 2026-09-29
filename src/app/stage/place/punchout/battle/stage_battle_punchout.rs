#[repr(C)]
pub struct StageBattlePunchOut {
    _0: [u8; 0x8C0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattlePunchOut>(), 0x8C0);
        assert_eq!(offset_of!(StageBattlePunchOut, _0), 0x0);
    }
}
