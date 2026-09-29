#[repr(C)]
pub struct StageAnimalIsland {
    _0: [u8; 0xE40],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Guest {
    /// Grams
    #[default]
    Geko,

    /// Tortimer
    Kotobuki,

    /// Leila
    Kuku,

    /// Leilani
    Kuuko,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub island: u8,
    pub guest: Guest,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageAnimalIsland>(), 0xE40);
        assert_eq!(offset_of!(StageAnimalIsland, _0), 0x0);
    }
}
