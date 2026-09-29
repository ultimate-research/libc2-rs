#[repr(C)]
pub struct GlobalMeleeParameter {
    pub melee_mode: MeleeMode,
    pub _1: u8,
    pub melee_rule_type: MeleeRuleType,
    pub _3: u8,
    _4: [u8; 0x1],
    pub _5: u8,
    pub _6: u8,
    pub _7: u8,
    pub _8: u8,
    pub is_super_sudden_death: bool,
    pub is_sudden_death: bool,
    pub _b: u8,
    _c: [u8; 0x1],
    pub _d: u8,
    pub _e: u8,
    _f: [u8; 0x1],
    pub _10: u8,
    pub is_melee_large: bool,
    _12: [u8; 0x2],
    pub is_team_attack: bool,
    pub _15: u8,
    _16: [u8; 0x1],
    pub _17: u8,
    _18: [u8; 0x5],
    pub _1d: u8,
    _1e: [u8; 0x1],
    pub _1f: u8,
    _20: [u8; 0x3],
    pub _23: u8,
    _24: [u8; 0x3],
    pub _27: u8,
    pub is_spirits_battle: bool,
    pub _29: u8,
    pub _2a: u8,
    pub _2b: u8,
    _2c: [u8; 0x4],
    pub _30: u8,
    _31: [u8; 0x5],
    pub _36: u8,
    pub _37: u8,
    pub is_team_battle: bool,
    pub _39: u8,
    pub _3a: u8,
    pub _3b: u8,
    _3c: [u8; 0xC],
    pub battle_frame_limit: i32,
    pub battle_stock: i32,
    _50: [u8; 0x4],
    pub item_appear: ItemAppear,
    _55: [u8; 0x63],
    pub is_boss_battle: bool,
    _b9: [u8; 0x367],
}

impl GlobalMeleeParameter {
    /// Returns a reference to the current melee mode.
    pub fn melee_mode(&self) -> &MeleeMode {
        &self.melee_mode
    }

    /// Returns `true` if the current melee mode is Training.
    pub fn is_training_mode(&self) -> bool {
        matches!(
            self.melee_mode,
            MeleeMode::TrainingSandbag | MeleeMode::SingleTrainingOffline
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MeleeMode {
    Normal,
    NormalMulti,
    TeamCompe,
    TeamCompeMulti,
    Standard,
    StandardMulti,
    AllBattle,
    SpiritsBattle,
    SpiritsBattleMulti,
    ContestHomerun,
    ContestKumite,
    ContestKumiteMulti,
    TrainingSandbag,
    ShamBattle,
    AnyoneSingle,
    AnyoneDouble,
    AnyoneVIP,
    AnyoneAmiibo,
    AnyoneSpirits,
    FriendSpirits,
    AnyoneRoom,
    FriendRoom,
    AnyoneCoop,
    FriendCoop,
    OnlineTournament,
    OnlineWatching,
    OnlineConquest,
    Unknown1,
    MenuFighter,
    Unknown2,
    GameplayDemo,
    StaffRoll,
    SingleTrainingOffline,
    SingleHowTo,
    MasterHandOperationalGame,
    Vr,
    VrMulti,
    OnlineConvention,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MeleeRuleType {
    Time,
    Stock,
    HP,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ItemAppear {
    None,
    Low,
    Normal,
    High,
    Highest,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(offset_of!(GlobalMeleeParameter, melee_mode), 0x0);
        assert_eq!(offset_of!(GlobalMeleeParameter, _1), 0x1);
        assert_eq!(offset_of!(GlobalMeleeParameter, melee_rule_type), 0x2);
        assert_eq!(offset_of!(GlobalMeleeParameter, _3), 0x3);
        assert_eq!(offset_of!(GlobalMeleeParameter, _5), 0x5);
        assert_eq!(offset_of!(GlobalMeleeParameter, _6), 0x6);
        assert_eq!(offset_of!(GlobalMeleeParameter, _7), 0x7);
        assert_eq!(offset_of!(GlobalMeleeParameter, _8), 0x8);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_super_sudden_death), 0x9);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_sudden_death), 0xA);
        assert_eq!(offset_of!(GlobalMeleeParameter, _b), 0xB);
        assert_eq!(offset_of!(GlobalMeleeParameter, _d), 0xD);
        assert_eq!(offset_of!(GlobalMeleeParameter, _e), 0xE);
        assert_eq!(offset_of!(GlobalMeleeParameter, _10), 0x10);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_melee_large), 0x11);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_team_attack), 0x14);
        assert_eq!(offset_of!(GlobalMeleeParameter, _15), 0x15);
        assert_eq!(offset_of!(GlobalMeleeParameter, _17), 0x17);
        assert_eq!(offset_of!(GlobalMeleeParameter, _1d), 0x1D);
        assert_eq!(offset_of!(GlobalMeleeParameter, _1f), 0x1F);
        assert_eq!(offset_of!(GlobalMeleeParameter, _23), 0x23);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_spirits_battle), 0x28);
        assert_eq!(offset_of!(GlobalMeleeParameter, _29), 0x29);
        assert_eq!(offset_of!(GlobalMeleeParameter, _2a), 0x2a);
        assert_eq!(offset_of!(GlobalMeleeParameter, _2b), 0x2b);
        assert_eq!(offset_of!(GlobalMeleeParameter, _30), 0x30);
        assert_eq!(offset_of!(GlobalMeleeParameter, _36), 0x36);
        assert_eq!(offset_of!(GlobalMeleeParameter, _37), 0x37);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_team_battle), 0x38);
        assert_eq!(offset_of!(GlobalMeleeParameter, _39), 0x39);
        assert_eq!(offset_of!(GlobalMeleeParameter, _3a), 0x3A);
        assert_eq!(offset_of!(GlobalMeleeParameter, _3b), 0x3B);
        assert_eq!(offset_of!(GlobalMeleeParameter, battle_frame_limit), 0x48);
        assert_eq!(offset_of!(GlobalMeleeParameter, battle_stock), 0x4C);
        assert_eq!(offset_of!(GlobalMeleeParameter, _50), 0x50);
        assert_eq!(offset_of!(GlobalMeleeParameter, item_appear), 0x54);
        assert_eq!(offset_of!(GlobalMeleeParameter, _55), 0x55);
        assert_eq!(offset_of!(GlobalMeleeParameter, is_boss_battle), 0xB8);
        assert_eq!(offset_of!(GlobalMeleeParameter, _b9), 0xB9);

        assert_eq!(size_of::<MeleeMode>(), 0x1);
        assert_eq!(MeleeMode::Normal as u8, 0x0);
        assert_eq!(MeleeMode::NormalMulti as u8, 0x1);
        assert_eq!(MeleeMode::TeamCompe as u8, 0x2);
        assert_eq!(MeleeMode::TeamCompeMulti as u8, 0x3);
        assert_eq!(MeleeMode::Standard as u8, 0x4);
        assert_eq!(MeleeMode::StandardMulti as u8, 0x5);
        assert_eq!(MeleeMode::AllBattle as u8, 0x6);
        assert_eq!(MeleeMode::SpiritsBattle as u8, 0x7);
        assert_eq!(MeleeMode::SpiritsBattleMulti as u8, 0x8);
        assert_eq!(MeleeMode::ContestHomerun as u8, 0x9);
        assert_eq!(MeleeMode::ContestKumite as u8, 0xA);
        assert_eq!(MeleeMode::ContestKumiteMulti as u8, 0xB);
        assert_eq!(MeleeMode::TrainingSandbag as u8, 0xC);
        assert_eq!(MeleeMode::ShamBattle as u8, 0xD);
        assert_eq!(MeleeMode::AnyoneSingle as u8, 0xE);
        assert_eq!(MeleeMode::AnyoneDouble as u8, 0xF);
        assert_eq!(MeleeMode::AnyoneVIP as u8, 0x10);
        assert_eq!(MeleeMode::AnyoneAmiibo as u8, 0x11);
        assert_eq!(MeleeMode::AnyoneSpirits as u8, 0x12);
        assert_eq!(MeleeMode::FriendSpirits as u8, 0x13);
        assert_eq!(MeleeMode::AnyoneRoom as u8, 0x14);
        assert_eq!(MeleeMode::FriendRoom as u8, 0x15);
        assert_eq!(MeleeMode::AnyoneCoop as u8, 0x16);
        assert_eq!(MeleeMode::FriendCoop as u8, 0x17);
        assert_eq!(MeleeMode::OnlineTournament as u8, 0x18);
        assert_eq!(MeleeMode::OnlineWatching as u8, 0x19);
        assert_eq!(MeleeMode::OnlineConquest as u8, 0x1A);
        assert_eq!(MeleeMode::Unknown1 as u8, 0x1B);
        assert_eq!(MeleeMode::MenuFighter as u8, 0x1C);
        assert_eq!(MeleeMode::Unknown2 as u8, 0x1D);
        assert_eq!(MeleeMode::GameplayDemo as u8, 0x1E);
        assert_eq!(MeleeMode::StaffRoll as u8, 0x1F);
        assert_eq!(MeleeMode::SingleTrainingOffline as u8, 0x20);
        assert_eq!(MeleeMode::SingleHowTo as u8, 0x21);
        assert_eq!(MeleeMode::MasterHandOperationalGame as u8, 0x22);
        assert_eq!(MeleeMode::Vr as u8, 0x23);
        assert_eq!(MeleeMode::VrMulti as u8, 0x24);
        assert_eq!(MeleeMode::OnlineConvention as u8, 0x25);

        assert_eq!(size_of::<MeleeRuleType>(), 0x1);
        assert_eq!(MeleeRuleType::Time as u8, 0x0);
        assert_eq!(MeleeRuleType::Stock as u8, 0x1);
        assert_eq!(MeleeRuleType::HP as u8, 0x2);

        assert_eq!(size_of::<ItemAppear>(), 0x1);
        assert_eq!(ItemAppear::None as u8, 0x0);
        assert_eq!(ItemAppear::Low as u8, 0x1);
        assert_eq!(ItemAppear::Normal as u8, 0x2);
        assert_eq!(ItemAppear::High as u8, 0x3);
        assert_eq!(ItemAppear::Highest as u8, 0x4);
    }
}
