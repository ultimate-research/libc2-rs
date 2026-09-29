#[repr(C)]
pub struct StageFzeroBigblue {
    _0: [u8; 0x950],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFzeroBigblue>(), 0x950);
        assert_eq!(offset_of!(StageFzeroBigblue, _0), 0x0);
    }
}
