#[repr(C)]
pub struct StagePickelWorld {
    _0: [u8; 0xAB0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum TimeOfDaySpirits {
    #[default]
    Cycle,
    Daytime,
    Night,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum BiomeSpirits {
    #[default]
    Random,
    Plains,
    BirchForest,
    Savanna,
    Taiga,
    SnowyTundra,
    StoneShore,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum FarmAnimalSpirits {
    #[default]
    Random,
    Chicken,
    Cow,
    Sheep,
    None,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ExSetting {
    pub time_of_day: TimeOfDaySpirits,
    pub biome: BiomeSpirits,
    pub farm_animal: FarmAnimalSpirits,
    pub is_hide_monster: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Biome {
    Plains,
    BirchForest,
    Savanna,
    Taiga,
    SnowyTundra,
    StoneShore,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FarmAnimal {
    Sheep,
    Chicken,
    Cow,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub world_info: u8,
}

impl RandomSetting {
    pub const fn new(biome: Biome, farm_animal: FarmAnimal) -> Self {
        Self {
            world_info: biome as u8 | (farm_animal as u8) << 3,
        }
    }

    pub const fn biome(self) -> Option<Biome> {
        match self.world_info & 0b00000111 {
            0 => Some(Biome::Plains),
            1 => Some(Biome::BirchForest),
            2 => Some(Biome::Savanna),
            3 => Some(Biome::Taiga),
            4 => Some(Biome::SnowyTundra),
            5 => Some(Biome::StoneShore),
            _ => None,
        }
    }

    pub const fn farm_animal(self) -> Option<FarmAnimal> {
        match (self.world_info & 0b00011000) >> 3 {
            0 => Some(FarmAnimal::Sheep),
            1 => Some(FarmAnimal::Chicken),
            2 => Some(FarmAnimal::Cow),
            _ => None,
        }
    }
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
