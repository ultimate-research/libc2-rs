#[repr(C)]
pub struct StageMarioPastX {
    _0: [u8; 0x910],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioPastX>(), 0x910);
        assert_eq!(offset_of!(StageMarioPastX, _0), 0x0);
    }
}
