#[repr(C)]
pub struct StageBattleDollyStadium {
    _0: [u8; 0x850],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleDollyStadium>(), 0x850);
        assert_eq!(offset_of!(StageBattleDollyStadium, _0), 0x0);
    }
}
