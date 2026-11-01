#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    /// Determines if the stage should start on Dive to the Heart.
    pub is_dive_fix: bool,

    /// The selected stained-glass design for Dive to the Heart.
    pub scene: Scene,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The selected stained-glass design for Dive to the Heart.
    pub scene: Scene,
}

/// A numeric identifier for a stained-glass design.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Scene {
    /// Random
    #[default]
    Random,

    /// Sora
    Sora,

    /// Riku
    Riku,

    /// Roxas
    Roxas,

    /// Xion
    Xion,

    /// Terra
    Terra,

    /// Ventus
    Ventus,

    /// Aqua
    Aqua,
}
