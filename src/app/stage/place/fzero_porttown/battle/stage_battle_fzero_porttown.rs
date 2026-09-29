#[repr(C)]
pub struct StageBattleFzeroPorttown {
    _0: [u8; 0x820],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFzeroPorttown>(), 0x820);
        assert_eq!(offset_of!(StageBattleFzeroPorttown, _0), 0x0);
    }
}
