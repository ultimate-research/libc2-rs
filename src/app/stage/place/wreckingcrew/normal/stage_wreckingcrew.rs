#[repr(C)]
pub struct StageWreckingCrew {
    _0: [u8; 0xB00],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageWreckingCrew>(), 0xB00);
        assert_eq!(offset_of!(StageWreckingCrew, _0), 0x0);
    }
}
