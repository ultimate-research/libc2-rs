#[repr(C)]
pub struct StagePikminGarden {
    _0: [u8; 0xB30],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePikminGarden>(), 0xB30);
        assert_eq!(offset_of!(StagePikminGarden, _0), 0x0);
    }
}
