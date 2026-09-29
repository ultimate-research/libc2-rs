#[repr(C)]
pub struct StageNintendogs {
    _0: [u8; 0x740],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Pet {
    #[default]
    Beagle,
    ToyPoodle,
    ShibaInu,
    JackRussellTerrier,
    GoldenRetriever,
    CalicoCat,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub pet: Pet,
    pub _1: u8,
    pub _2: u8,
    pub _3: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageNintendogs>(), 0x740);
        assert_eq!(offset_of!(StageNintendogs, _0), 0x0);
    }
}
