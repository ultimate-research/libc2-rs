#[repr(C)]
pub struct StageMarioBros {
    _0: [u8; 0xBD0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioBros>(), 0xBD0);
        assert_eq!(offset_of!(StageMarioBros, _0), 0x0);
    }
}
