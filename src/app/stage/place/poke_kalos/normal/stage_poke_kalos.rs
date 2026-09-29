#[repr(C)]
pub struct StagePokeKalos {
    _0: [u8; 0x14A0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Room {
    #[default]
    Random,
    FlameTransition,
    WaterTransition,
    SteelTransition,
    DragonTransition,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Pokemon {
    #[default]
    Random,
    Rare,
    Normal,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    pub room: Room,
    pub pokemon: Pokemon,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeKalos>(), 0x14A0);
        assert_eq!(offset_of!(StagePokeKalos, _0), 0x0);
    }
}
