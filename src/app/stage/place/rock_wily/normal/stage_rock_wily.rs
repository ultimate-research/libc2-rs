#[repr(C)]
pub struct StageRockWily {
    _0: [u8; 0x9C0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageRockWily>(), 0x9C0);
        assert_eq!(offset_of!(StageRockWily, _0), 0x0);
    }
}
