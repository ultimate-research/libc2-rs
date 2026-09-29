#[repr(C)]
pub struct StageIcarusUprising {
    _0: [u8; 0xAA0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageIcarusUprising>(), 0xAA0);
        assert_eq!(offset_of!(StageIcarusUprising, _0), 0x0);
    }
}
