#[repr(C)]
pub struct StageFEArena {
    _0: [u8; 0x930],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFEArena>(), 0x930);
        assert_eq!(offset_of!(StageFEArena, _0), 0x0);
    }
}
