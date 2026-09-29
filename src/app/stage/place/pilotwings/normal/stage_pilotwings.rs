#[repr(C)]
pub struct StagePilotwings {
    _0: [u8; 0x860],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePilotwings>(), 0x860);
        assert_eq!(offset_of!(StagePilotwings, _0), 0x0);
    }
}
