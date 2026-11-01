#[repr(C)]
pub struct StageSPEdit {
    _0: [u8; 0xBB0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The selected background for the stage.
    pub bg: Bg,
}

/// A numeric identifier for the background of a stage.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Bg {
    /// Battlefield
    #[default]
    BattleField,

    /// Tomodachi Life
    Tomodachi,

    /// Wrecking Crew
    WreckingCrew,

    /// Kalos Pokémon League
    PokeKalos,

    /// Coliseum
    FEColloseum,

    /// Find Mii
    StreetPass,

    /// Summit
    IceTop,

    /// Final Destination
    End,

    /// Magicant
    MotherMagicant,

    /// Luigi's Mansion
    LuigiMansion,

    /// Rainbow Cruise
    MarioRainbow,

    /// Mario Galaxy
    MarioGalaxy,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSPEdit>(), 0xBB0);
        assert_eq!(offset_of!(StageSPEdit, _0), 0x0);
    }
}
