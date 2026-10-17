/// A numeric identifier for altering the selected stage based on controller inputs.
#[derive(Debug)]
#[repr(C)]
pub enum SecretCommandID {
    /// L Button (SL Button with a sideways Joy-Con.)
    L,

    /// R Button (SR Button with a sideways Joy-Con.)
    R,

    /// L and R Buttons (SL and SR Buttons with a sideways Joy-Con.)
    LR,

    /// Exclusive to the Minecraft World stage.
    Extra,

    /// None
    None,
}
