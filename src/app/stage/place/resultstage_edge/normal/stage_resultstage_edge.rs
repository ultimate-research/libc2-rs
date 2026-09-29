#[repr(C)]
pub struct StageResultStageEdge {
    _0: [u8; 0x750],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageResultStageEdge>(), 0x750);
        assert_eq!(offset_of!(StageResultStageEdge, _0), 0x0);
    }
}
