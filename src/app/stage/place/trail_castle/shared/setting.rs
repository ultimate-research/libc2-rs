#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Scene {
    #[default]
    Random,
    Sora,
    Riku,
    Roxas,
    Xion,
    Terra,
    Ventus,
    Aqua,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    /// Determines if the stage should start on Dive to the Heart.
    pub is_dive_fix: bool,

    /// The stained-glass design that should appear.
    pub scene: Scene,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The stained-glass design that should appear.
    pub scene: Scene,
}
