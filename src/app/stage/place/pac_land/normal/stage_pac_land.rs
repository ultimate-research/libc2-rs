#[repr(C)]
pub struct StagePacLand {
    _0: [u8; 0xC40],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePacLand>(), 0xC40);
        assert_eq!(offset_of!(StagePacLand, _0), 0x0);
    }
}
