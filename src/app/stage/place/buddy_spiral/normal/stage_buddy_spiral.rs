#[repr(C)]
pub struct StageBuddySpiral {
    _0: [u8; 0xB50],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBuddySpiral>(), 0xB50);
        assert_eq!(offset_of!(StageBuddySpiral, _0), 0x0);
    }
}
