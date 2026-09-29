#[repr(C)]
pub struct StageWiiFit {
    _0: [u8; 0x910],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageWiiFit>(), 0x910);
        assert_eq!(offset_of!(StageWiiFit, _0), 0x0);
    }
}
