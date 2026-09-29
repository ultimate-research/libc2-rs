#[repr(C)]
pub struct StageLuigiMansion {
    _0: [u8; 0x860],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageLuigiMansion>(), 0x860);
        assert_eq!(offset_of!(StageLuigiMansion, _0), 0x0);
    }
}
