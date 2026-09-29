#[repr(C)]
pub struct StageAnimalVillage {
    _0: [u8; 0x890],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageAnimalVillage>(), 0x890);
        assert_eq!(offset_of!(StageAnimalVillage, _0), 0x0);
    }
}
