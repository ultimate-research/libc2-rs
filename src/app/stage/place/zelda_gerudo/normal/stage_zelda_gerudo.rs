#[repr(C)]
pub struct StageZeldaGerudo {
    _0: [u8; 0x1240],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaGerudo>(), 0x1240);
        assert_eq!(offset_of!(StageZeldaGerudo, _0), 0x0);
    }
}
