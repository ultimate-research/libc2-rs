#[repr(C)]
pub struct StageFESiege {
    _0: [u8; 0x9B0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFESiege>(), 0x9B0);
        assert_eq!(offset_of!(StageFESiege, _0), 0x0);
    }
}
