#[repr(C)]
pub struct StageBattleMarioDolpic {
    _0: [u8; 0x780],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleMarioDolpic>(), 0x780);
        assert_eq!(offset_of!(StageBattleMarioDolpic, _0), 0x0);
    }
}
