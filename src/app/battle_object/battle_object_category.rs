#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BattleObjectCategory {
    /// Fighter
    Fighter,

    /// Weapon
    Weapon,

    /// Enemy
    Enemy,

    /// Stage Hazard
    Gimmick,

    /// Item
    Item,
}
