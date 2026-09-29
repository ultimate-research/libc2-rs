#[repr(C)]
pub struct StageBalloonFight {
    _0: [u8; 0xCF0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub level: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBalloonFight>(), 0xCF0);
        assert_eq!(offset_of!(StageBalloonFight, _0), 0x0);
    }
}
