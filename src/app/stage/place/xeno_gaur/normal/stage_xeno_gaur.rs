#[repr(C)]
pub struct StageXenoGaur {
    _0: [u8; 0xAB0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageXenoGaur>(), 0xAB0);
        assert_eq!(offset_of!(StageXenoGaur, _0), 0x0);
    }
}
