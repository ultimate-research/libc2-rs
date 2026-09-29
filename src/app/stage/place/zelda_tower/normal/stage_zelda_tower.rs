#[repr(C)]
pub struct StageZeldaTower {
    _0: [u8; 0x850],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaTower>(), 0x850);
        assert_eq!(offset_of!(StageZeldaTower, _0), 0x0);
    }
}
