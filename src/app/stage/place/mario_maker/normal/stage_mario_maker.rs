#[repr(C)]
pub struct StageMarioMaker {
    _0: [u8; 0x9F0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub _0: u8,
    pub _1: u8,
    pub _2: u8,
    pub _3: u8,
    pub _4: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioMaker>(), 0x9F0);
        assert_eq!(offset_of!(StageMarioMaker, _0), 0x0);
    }
}
