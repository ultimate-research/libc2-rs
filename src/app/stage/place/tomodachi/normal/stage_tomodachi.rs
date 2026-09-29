#[repr(C)]
pub struct StageTomodachi {
    _0: [u8; 0xF90],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTomodachi>(), 0xF90);
        assert_eq!(offset_of!(StageTomodachi, _0), 0x0);
    }
}
