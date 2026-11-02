#[repr(C)]
pub struct StagePickelWorld {
    _0: [u8; 0xAB0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    /// The selected time of day.
    pub time_of_day: TimeOfDayEx,

    /// The selected biome.
    pub biome: BiomeEx,

    /// The selected passive guest character.
    pub guest: GuestEx,

    /// Determines if hostile mobs should not appear.
    pub is_hide_monster: bool,
}

/// A numeric identifier for the time of day.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum TimeOfDayEx {
    /// Day-night cycle
    #[default]
    Cycle,

    /// Daytime
    Daytime,

    /// Night
    Night,
}

/// A numeric identifier for a biome with additional general variants.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum BiomeEx {
    /// Random
    #[default]
    Random,

    /// Plains
    Plains,

    /// Birch Forest
    BirchForest,

    /// Savanna
    Savanna,

    /// Taiga
    Taiga,

    /// Snowy Tundra
    SnowyTundra,

    /// Stone Shore
    StoneShore,
}

/// A numeric identifier for a passive guest character with additional general variants.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum GuestEx {
    /// Random
    #[default]
    Random,

    /// Chicken
    Chicken,

    /// Cow
    Cow,

    /// Sheep
    Sheep,

    /// None
    None,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The set of configured options for the world.
    pub world_info: WorldInfo,
}

/// A set of configurable options for the world.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct WorldInfo {
    inner: u8,
}

impl WorldInfo {
    /// The bitmask for the raw value corresponding to the selected biome.
    const BIOME_MASK: u8 = 0b00000111;

    /// The bitmask for the raw value corresponding to the selected passive guest character.
    const GUEST_MASK: u8 = 0b00011000;

    /// The right-hand shift operand for extracting the raw value corresponding to the selected passive guest character.
    const GUEST_SHIFT: u32 = Self::GUEST_MASK.leading_zeros();

    /// Creates a new `WorldInfo`.
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Creates a new `WorldInfo` with the given biome.
    pub const fn with_biome(mut self, biome: Biome) -> Self {
        self.set_biome(biome);
        self
    }

    /// Creates a new `WorldInfo` with the given guest character.
    pub const fn with_guest(mut self, guest: Guest) -> Self {
        self.set_guest(guest);
        self
    }

    /// Extracts the selected biome.
    pub const fn biome(&self) -> Option<Biome> {
        match self.inner & Self::BIOME_MASK {
            0 => Some(Biome::Plains),
            1 => Some(Biome::BirchForest),
            2 => Some(Biome::Savanna),
            3 => Some(Biome::Taiga),
            4 => Some(Biome::SnowyTundra),
            5 => Some(Biome::StoneShore),
            _ => None,
        }
    }

    /// Extracts the selected passive guest character.
    pub const fn guest(&self) -> Option<Guest> {
        match (self.inner & Self::GUEST_MASK) >> Self::GUEST_SHIFT {
            0 => Some(Guest::Sheep),
            1 => Some(Guest::Chicken),
            2 => Some(Guest::Cow),
            _ => None,
        }
    }

    /// Assigns the given biome to the selected biome.
    pub const fn set_biome(&mut self, biome: Biome) {
        self.inner = (self.inner & !Self::BIOME_MASK) | biome as u8;
    }

    /// Assigns the given passive guest character to the selected passive guest character.
    pub const fn set_guest(&mut self, guest: Guest) {
        self.inner = (self.inner & !Self::GUEST_MASK) | ((guest as u8) << Self::GUEST_SHIFT);
    }

    /// Consumes the `WorldInfo`, returning the underlying value.
    pub const fn into_inner(self) -> u8 {
        self.inner
    }
}

/// A numeric identifier for a biome.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Biome {
    /// Plains
    Plains,

    /// Birch Forest
    BirchForest,

    /// Savanna
    Savanna,

    /// Taiga
    Taiga,

    /// Snowy Tundra
    SnowyTundra,

    /// Stone Shore
    StoneShore,
}

/// A numeric identifier for a passive guest character.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Guest {
    /// Sheep
    Sheep,

    /// Chicken
    Chicken,

    /// Cow
    Cow,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePickelWorld>(), 0xAB0);
        assert_eq!(offset_of!(StagePickelWorld, _0), 0x0);
    }
}
