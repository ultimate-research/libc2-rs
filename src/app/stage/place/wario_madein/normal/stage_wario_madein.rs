#[repr(C)]
pub struct StageWarioMadein {
    _0: [u8; 0x790],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageWarioMadein>(), 0x790);
        assert_eq!(offset_of!(StageWarioMadein, _0), 0x0);
    }
}
