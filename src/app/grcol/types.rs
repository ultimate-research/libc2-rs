use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    #[repr(C)]
    pub struct AttributeFlags: u32 {
        const LENGTH0 = 0x1;
        const PACKMAN_FINAL_IGNORE = 0x2;
        const FALL = 0x4;
        const IGNORE_RAY_CHECK = 0x8;
        const DIVE = 0x10;
        const UNPAINTABLE = 0x20;
        const ITEM = 0x40;
        const IGNORE_FIGHTER_OTHER = 0x80;
        const RIGHT = 0x100;
        const LEFT = 0x200;
        const UPPER = 0x400;
        const UNDER = 0x800;
        const NOT_ATTACH = 0x1000;
        const THROUGHABLE = 0x2000;
        const HANG_L = 0x4000;
        const HANG_R = 0x8000;
        const IGNORE_LINK_FROM_LEFT = 0x10000;
        const CLOUD = 0x20000;
        const IGNORE_LINK_FROM_RIGHT = 0x40000;
        const NOT_EXPAND_NEAR_SEARCH = 0x80000;
        const IGNORE = 0x100000;
        const BREAKABLE = 0x200000;
        const IMMEDIATE_RELANDING_BAN = 0x400000;
        const IGNORE_LINE_TYPE1 = 0x800000;
        const PICKEL_BLOCK = 0x1000000;
        const DECELERATION = 0x2000000;
        const VIRTUAL_HIT_LINE_UP = 0x4000000;
        const VIRTUAL_HIT_LINE_LEFT = 0x8000000;
        const VIRTUAL_HIT_LINE_RIGHT = 0x10000000;
        const VIRTUAL_HIT_LINE_DOWN = 0x20000000;
        const VIRTUAL_WALL_HIT_LINE = 0x40000000;
        const IGNORE_BOSS = 0x80000000;
        const VIRTUAL_HIT_LINE_ALL = Self::VIRTUAL_HIT_LINE_UP.bits() | Self::VIRTUAL_HIT_LINE_LEFT.bits() | Self::VIRTUAL_HIT_LINE_RIGHT.bits() | Self::VIRTUAL_HIT_LINE_DOWN.bits();
    }
}

/// A numeric identifier for a ground collision material.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub enum AttrType {
    #[default]
    None,
    Rock,
    Grass,
    Soil,
    Wood,
    Iron,
    Nibuiron,
    Carpet,
    Numenume,
    Creature,
    Asase,
    Soft,
    Turuturu,
    Snow,
    Ice,
    Gamewatch,
    Oil,
    Danbouru,
    Damage1,
    Damage2,
    Damage3,
    Plankton,
    Cloud,
    Akuukan,
    Brick,
    Noattr,
    Mario,
    Wirenetting,
    Sand,
    Homerun,
    AsaseEarth,
    Death,
    Ringmat,
    Glass,
    Slipdx,
    SpPoison,
    SpFlame,
    SpElectricShock,
    SpSleep,
    SpFreezing,
    SpAdhesion,
    IceNoSlip,
    CloudNoThrough,
    JackMementoes,
}
