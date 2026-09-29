#[repr(C)]
pub struct StageIcarusAngeland {
    _0: [u8; 0xA90],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageIcarusAngeland>(), 0xA90);
        assert_eq!(offset_of!(StageIcarusAngeland, _0), 0x0);
    }
}
