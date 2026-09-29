#[repr(C)]
pub struct StageYoshiCartBoard {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageYoshiCartBoard>(), 0x740);
        assert_eq!(offset_of!(StageYoshiCartBoard, _0), 0x0);
    }
}
