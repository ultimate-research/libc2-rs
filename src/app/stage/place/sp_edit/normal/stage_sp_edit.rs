#[repr(C)]
pub struct StageSPEdit {
    _0: [u8; 0xBB0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Bg {
    #[default]
    BattleField,
    Tomodachi,
    WreckingCrew,
    PokeKalos,
    FEColloseum,
    StreetPass,
    IceTop,
    End,
    MotherMagicant,
    LuigiMansion,
    MarioRainbow,
    MarioGalaxy,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub bg: Bg,
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
