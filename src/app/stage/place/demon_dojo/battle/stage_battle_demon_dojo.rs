#[repr(C)]
pub struct StageBattleDemonDojo {
    _0: [u8; 0x770],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleDemonDojo>(), 0x770);
        assert_eq!(offset_of!(StageBattleDemonDojo, _0), 0x0);
    }
}
