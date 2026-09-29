#[repr(C)]
pub struct StageFFMidgar {
    _0: [u8; 0x1C80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFFMidgar>(), 0x1C80);
        assert_eq!(offset_of!(StageFFMidgar, _0), 0x0);
    }
}
