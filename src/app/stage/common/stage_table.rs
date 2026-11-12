use crate::{
    app::{StageID, StagePlaceID},
    phx::Hash40,
};

/// A collection of correspondance information for each [`StageID`].
#[repr(C)]
pub struct StageTable {
    inner: [StageTableEntry; StageID::COUNT],
}

#[cfg(feature = "impl")]
impl StageTable {
    /// Returns a reference to the global instance of `StageTable`.
    pub fn instance() -> &'static Self {
        use skyline::hooks::{Region, getRegionAddress};

        unsafe {
            &*getRegionAddress(Region::Text)
                .byte_offset(0x454A7E8)
                .cast::<Self>()
        }
    }

    /// Returns a reference to the [`StageTableEntry`] corresponding to the given identifier.
    pub fn get(&self, id: StageID) -> &StageTableEntry {
        &self.inner[id as usize]
    }
}

/// The underlying element type for a [`StageTable`].
#[derive(Debug)]
#[repr(C)]
pub struct StageTableEntry {
    // TODO: Field documentation.
    pub id: StageID,

    /// The corresponding umbrella identifier.
    pub place_id: StagePlaceID,

    /// The form of the stage.
    pub ty: StageType,

    /// The hashed name of the identifier.
    pub name: Hash40,

    /// The hashed path to the resources directory.
    pub resource_path: Hash40, // "stage/%s/%s"

    /// The hashed path to the particle effects directory.
    pub effect_path: Hash40, // "effect/stage/%s"

    /// The hashed path to the sound effects NUS3BANK file.
    pub sound_bank_path: Hash40, // "sound/bank/stage/se_stage_%s.nus3bank"

    /// The hashed path to the sound effects SQB file.
    pub sound_sequence_path: Hash40, // "sound/sequence/stage/%s.sqb"

    /// The hashed path to the sound effects NUS3AUDIO file.
    pub sound_audio_path: Hash40, // "sound/bank/stage/se_stage_%s.nus3audio"

    /// The hashed path to the sound effects TONELABEL file.
    pub sound_tonelabel_path: Hash40, // "sound/bank/stage/se_stage_%s.tonelabel"
}

/// A numeric identifier for a form of a stage.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum StageType {
    /// Normal
    Normal,

    /// Ω Form
    End,

    /// Battlefield Form
    Battle,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTableEntry>(), 0x48);
        assert_eq!(offset_of!(StageTableEntry, id), 0x0);
        assert_eq!(offset_of!(StageTableEntry, place_id), 0x4);
        assert_eq!(offset_of!(StageTableEntry, ty), 0x8);
        assert_eq!(offset_of!(StageTableEntry, name), 0x10);
        assert_eq!(offset_of!(StageTableEntry, resource_path), 0x18);
        assert_eq!(offset_of!(StageTableEntry, effect_path), 0x20);
        assert_eq!(offset_of!(StageTableEntry, sound_bank_path), 0x28);
        assert_eq!(offset_of!(StageTableEntry, sound_sequence_path), 0x30);
        assert_eq!(offset_of!(StageTableEntry, sound_audio_path), 0x38);
        assert_eq!(offset_of!(StageTableEntry, sound_tonelabel_path), 0x40);
    }
}
