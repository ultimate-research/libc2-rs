#[repr(C)]
pub struct StageZeldaSkyward {
    _0: [u8; 0xA80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaSkyward>(), 0xA80);
        assert_eq!(offset_of!(StageZeldaSkyward, _0), 0x0);
    }
}
