use crate::{app::StagePlaceID, phx::Hash40};

/// A collection of correspondance information for each [`StagePlaceID`].
#[repr(C)]
pub struct StagePlaceTable {
    inner: [StagePlaceTableEntry; StagePlaceID::COUNT],
}

#[cfg(feature = "impl")]
impl StagePlaceTable {
    /// Returns a reference to the global instance of `StagePlaceTable`.
    pub fn instance() -> &'static Self {
        use skyline::hooks::{Region, getRegionAddress};

        unsafe {
            &*getRegionAddress(Region::Text)
                .byte_offset(0x4549258)
                .cast::<Self>()
        }
    }

    /// Returns a reference to the [`StagePlaceTableEntry`] corresponding to the given identifier.
    pub fn get(&self, id: StagePlaceID) -> &StagePlaceTableEntry {
        &self.inner[id as usize]
    }
}

/// The underlying element type for a [`StagePlaceTable`].
#[derive(Debug)]
#[repr(C)]
pub struct StagePlaceTableEntry {
    // TODO: Field documentation.
    pub id: StagePlaceID,

    // TODO: Field documentation.
    pub ty: StagePlaceType,

    /// The hashed name of the identifier.
    pub name: Hash40,

    /// The hashed name of the resources directory.
    pub resource_name: Hash40,

    // TODO: Field documentation.
    pub _18: StagePlaceUnknown,

    /// The hashed internal name of the associated downloadable fighter.
    pub dlc_chara_name: Hash40,
}

// TODO: Type documentation.
#[derive(Debug)]
#[repr(C)]
pub enum StagePlaceType {
    // TODO: Field documentation.
    Normal,

    // TODO: Field documentation.
    Other,
}

// TODO: Type documentation.
#[derive(Debug)]
#[repr(C)]
pub enum StagePlaceUnknown {
    // TODO: Field documentation.
    Variant1 = 1,

    // TODO: Field documentation.
    Variant2 = 2,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePlaceTableEntry>(), 0x28);
        assert_eq!(offset_of!(StagePlaceTableEntry, id), 0x0);
        assert_eq!(offset_of!(StagePlaceTableEntry, ty), 0x4);
        assert_eq!(offset_of!(StagePlaceTableEntry, name), 0x8);
        assert_eq!(offset_of!(StagePlaceTableEntry, resource_name), 0x10);
        assert_eq!(offset_of!(StagePlaceTableEntry, _18), 0x18);
        assert_eq!(offset_of!(StagePlaceTableEntry, dlc_chara_name), 0x20);
    }
}
