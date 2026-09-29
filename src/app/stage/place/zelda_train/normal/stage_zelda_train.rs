#[repr(C)]
pub struct StageZeldaTrain {
    _0: [u8; 0xD80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaTrain>(), 0xD80);
        assert_eq!(offset_of!(StageZeldaTrain, _0), 0x0);
    }
}
