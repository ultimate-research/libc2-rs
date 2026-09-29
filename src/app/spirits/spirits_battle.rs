use crate::{app::StageID, phx::Hash40};

#[repr(C)]
pub struct SpiritsBattleData {
    vtable: usize,
    _8: u8,
    pub battle_id: Hash40,
    _18: u8,
    pub battle_type: SpiritsBattleType,
    pub battle_time_sec: u16,
    pub basic_init_damage: u16,
    pub basic_init_hp: u16,
    pub basic_stock: u8,
    pub stage_id: StageID,
    pub stage_bgm: Hash40,
    pub stage_additional_setting: i8,
    pub stage_gimmick: bool,
    _32: u8,
    pub stage_attr: StageAttr,
    pub floor_place_id: Hash40,
    pub item_table: Hash40,
    pub item_level: u8,
    pub result_type: u8,
    _4a: [u8; 0x12E],
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SpiritsBattleType {
    /// Timed Battle
    Time,

    /// Stock Battle
    Stock,

    /// Stamina Battle
    Hp,

    /// Timed Stock Battle
    TimeStock,

    /// Timed Stamina Battle
    TimeHp,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub enum StageAttr {
    /// None
    #[default]
    None,

    /// Poison Floor
    PoisonFloor,

    /// Lava Floor
    DamageFloor,

    /// Zap Floor
    ElecFloor,

    /// Sleep Floor
    SleepFloor,

    /// Ice Floor
    IceFloor,

    /// Sticky Floor
    GumFloor,

    /// Poison Cloud
    PoisonArea,

    /// Heavy Wind
    WindArea,

    /// Fog
    MistArea,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<SpiritsBattleData>(), 0x178);
        assert_eq!(offset_of!(SpiritsBattleData, vtable), 0x0);
        assert_eq!(offset_of!(SpiritsBattleData, _8), 0x8);
        assert_eq!(offset_of!(SpiritsBattleData, battle_id), 0x10);
        assert_eq!(offset_of!(SpiritsBattleData, _18), 0x18);
        assert_eq!(offset_of!(SpiritsBattleData, battle_type), 0x19);
        assert_eq!(offset_of!(SpiritsBattleData, battle_time_sec), 0x1A);
        assert_eq!(offset_of!(SpiritsBattleData, basic_init_damage), 0x1C);
        assert_eq!(offset_of!(SpiritsBattleData, basic_init_hp), 0x1E);
        assert_eq!(offset_of!(SpiritsBattleData, basic_stock), 0x20);
        assert_eq!(offset_of!(SpiritsBattleData, stage_id), 0x24);
        assert_eq!(offset_of!(SpiritsBattleData, stage_bgm), 0x28);
        assert_eq!(offset_of!(SpiritsBattleData, stage_additional_setting), 0x30);
        assert_eq!(offset_of!(SpiritsBattleData, stage_gimmick), 0x31);
        assert_eq!(offset_of!(SpiritsBattleData, _32), 0x32);
        assert_eq!(offset_of!(SpiritsBattleData, stage_attr), 0x34);
        assert_eq!(offset_of!(SpiritsBattleData, floor_place_id), 0x38);
        assert_eq!(offset_of!(SpiritsBattleData, item_table), 0x40);
        assert_eq!(offset_of!(SpiritsBattleData, item_level), 0x48);
        assert_eq!(offset_of!(SpiritsBattleData, result_type), 0x49);

        assert_eq!(size_of::<SpiritsBattleType>(), 0x1);
        assert_eq!(SpiritsBattleType::Time as u8, 0x0);
        assert_eq!(SpiritsBattleType::Stock as u8, 0x1);
        assert_eq!(SpiritsBattleType::Hp as u8, 0x2);
        assert_eq!(SpiritsBattleType::TimeStock as u8, 0x3);
        assert_eq!(SpiritsBattleType::TimeHp as u8, 0x4);

        assert_eq!(size_of::<StageAttr>(), 0x4);
        assert_eq!(StageAttr::None as i32, 0x0);
        assert_eq!(StageAttr::PoisonFloor as i32, 0x1);
        assert_eq!(StageAttr::DamageFloor as i32, 0x2);
        assert_eq!(StageAttr::ElecFloor as i32, 0x3);
        assert_eq!(StageAttr::SleepFloor as i32, 0x4);
        assert_eq!(StageAttr::IceFloor as i32, 0x5);
        assert_eq!(StageAttr::GumFloor as i32, 0x6);
        assert_eq!(StageAttr::PoisonArea as i32, 0x7);
        assert_eq!(StageAttr::WindArea as i32, 0x8);
        assert_eq!(StageAttr::MistArea as i32, 0x9);
    }
}
