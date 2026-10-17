/// The Hollow Bastion stage.
#[repr(C)]
pub struct StageTrailCastle {
    _0: [u8; 0xA40],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTrailCastle>(), 0xA40);
        assert_eq!(offset_of!(StageTrailCastle, _0), 0x0);
    }
}
