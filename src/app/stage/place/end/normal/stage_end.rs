use crate::app::StageBattleBase;

#[repr(C, align(16))]
pub struct StageEnd {
    pub base: StageBattleBase,
    _770: [u8; 0x1B0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum LandscapeSpirits {
    #[default]
    Random,
    Median,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    /// The background that should appear and remain fixed on.
    pub landscape: LandscapeSpirits,

    /// Determines if the decorations atop the surface of the stage should not appear.
    pub is_hide_ring_decorations: bool,

    /// Determines if the four floating platforms should appear.
    pub is_enable_floating_4plate: bool,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Landscape {
    #[default]
    Median,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub landscape: Landscape,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageEnd>(), 0x920);
        assert_eq!(offset_of!(StageEnd, base), 0x0);
        assert_eq!(offset_of!(StageEnd, _770), 0x770);
    }
}
