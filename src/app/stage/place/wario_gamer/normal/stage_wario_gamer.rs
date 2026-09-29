#[repr(C)]
pub struct StageWarioGamer {
    _0: [u8; 0xCF30],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageWarioGamer>(), 0xCF30);
        assert_eq!(offset_of!(StageWarioGamer, _0), 0x0);
    }
}
