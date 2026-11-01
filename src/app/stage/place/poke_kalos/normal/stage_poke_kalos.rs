#[repr(C)]
pub struct StagePokeKalos {
    _0: [u8; 0x14A0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    /// The selected chamber that can be switched to.
    pub room: RoomEx,

    /// The selected classification of Pokémon.
    pub pokemon: PokemonEx,
}

/// A numeric identifier for a chamber that can be switched to.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum RoomEx {
    /// Random
    #[default]
    Random,

    /// Fire-type Chamber
    FlameTransition,

    /// Water-type Chamber
    WaterTransition,

    /// Steel-type Chamber
    SteelTransition,

    /// Dragon-type Chamber
    DragonTransition,
}

/// A numeric identifier for a classification of Pokémon.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum PokemonEx {
    /// Random
    #[default]
    Random,

    /// Legendary/Mythical
    Rare,

    /// Ordinary
    Normal,
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
