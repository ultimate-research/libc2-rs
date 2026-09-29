#[repr(C)]
pub struct StageSplaParking {
    _0: [u8; 0x840],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSplaParking>(), 0x840);
        assert_eq!(offset_of!(StageSplaParking, _0), 0x0);
    }
}
