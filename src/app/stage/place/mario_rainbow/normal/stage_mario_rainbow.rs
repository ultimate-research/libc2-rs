#[repr(C)]
pub struct StageMarioRainbow {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioRainbow>(), 0x740);
        assert_eq!(offset_of!(StageMarioRainbow, _0), 0x0);
    }
}
