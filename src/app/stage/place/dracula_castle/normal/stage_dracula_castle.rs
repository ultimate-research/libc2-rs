#[repr(C)]
pub struct StageDraculaCastle {
    _0: [u8; 0x970],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDraculaCastle>(), 0x970);
        assert_eq!(offset_of!(StageDraculaCastle, _0), 0x0);
    }
}
