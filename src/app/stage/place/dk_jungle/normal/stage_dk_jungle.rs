#[repr(C)]
pub struct StageDKJungle {
    _0: [u8; 0x8B0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDKJungle>(), 0x8B0);
        assert_eq!(offset_of!(StageDKJungle, _0), 0x0);
    }
}
