use crate::cpp;

/// An observer for various scenarios related to components of battle.
#[repr(C)]
pub struct StageSituationLink {
    pub is_any_situation: bool,
    pub situation: [cpp::UniquePtr<StageSituationBase>; 4],
    pub situation_num: usize,
}

#[cfg(feature = "impl")]
impl StageSituationLink {
    /// Creates a new `StageSituationLink`.
    pub fn new(param: &StageSituationLinkCreateParam) -> Self {
        let mut this = core::mem::MaybeUninit::<Self>::uninit();

        // SAFETY: The C++ constructor will initialize the object for us.
        unsafe {
            this.assume_init_mut().create(param);
            this.assume_init()
        }
    }

    /// Creates a new `StageSituationLink` in place.
    fn create(&mut self, create_param: &StageSituationLinkCreateParam) {
        #[skyline::from_offset(0x317BC20)]
        fn inner(this: *mut StageSituationLink, create_param: &StageSituationLinkCreateParam);

        unsafe {
            inner(self, create_param);
        }
    }

    /// Updates the state of this `StageSituationLink`.
    pub fn update(&mut self) {
        for situation in &mut self.situation {
            situation.update();
        }

        self.is_any_situation = self.situation.iter().any(|s| s.is_situation());
    }

    /// Returns `true` if any one scenario is in effect.
    pub const fn is_any_situation(&self) -> bool {
        self.is_any_situation
    }
}

impl Drop for StageSituationLink {
    fn drop(&mut self) {
        for situation in &mut self.situation {
            if situation.is_null() {
                continue;
            }

            (situation.vtable.delete)(situation.as_mut_ptr());
        }
    }
}

/// The set of parameters corresponding to creating a new [`StageSituationLink`].
#[repr(C)]
pub struct StageSituationLinkCreateParam {
    /// Determines if the [`StageSituationLink`] can be affected by a fighter's stock count.
    pub is_link_stock: bool,

    /// Determines if the [`StageSituationLink`] can be affected by the remaining time.
    pub is_link_time: bool,

    /// Determines if the [`StageSituationLink`] can be affected by a fighter's stamina.
    pub is_link_hp: bool,

    /// The maximum required amount of time to put the corresponding scenario into effect, in seconds.
    pub remaining_time: f32,

    /// The maximum required ratio of a fighter's current stamina to their starting stamina to put the corresponding scenario into effect.
    pub hp_rate: f32,
}

impl StageSituationLinkCreateParam {
    /// Creates a new `StageSituationLinkCreateParam`.
    pub const fn new(
        is_link_stock: bool,
        is_link_time: bool,
        is_link_hp: bool,
        remaining_time: f32,
        hp_rate: f32,
    ) -> Self {
        Self {
            is_link_stock,
            is_link_time,
            is_link_hp,
            remaining_time,
            hp_rate,
        }
    }
}

impl Default for StageSituationLinkCreateParam {
    fn default() -> Self {
        Self::new(true, true, true, 30.0, 0.3)
    }
}

/// The interface for observing some scenario.
#[repr(C)]
pub struct StageSituationBase {
    vtable: &'static StageSituationBaseVtable,
}

impl StageSituationBase {
    /// Updates the state of this `StageSituationBase`.
    pub fn update(&mut self) {
        (self.vtable.update)(self);
    }

    /// Returns `true` if the observed scenario is in effect.
    pub fn is_situation(&self) -> bool {
        (self.vtable.is_situation)(self)
    }
}

#[repr(C)]
struct StageSituationBaseVtable {
    pub destroy: extern "C" fn(*mut StageSituationBase),
    pub delete: extern "C" fn(*mut StageSituationBase),
    pub update: extern "C" fn(*mut StageSituationBase),
    pub is_situation: extern "C" fn(*const StageSituationBase) -> bool,
}

/// An observer for a scenario where one or more fighters have a low number of stocks.
#[repr(C)]
pub struct StageSituationStock {
    pub base: StageSituationBase,

    /// Determines if the battle supports stocks.
    pub is_valid_stock: bool,

    /// Determines if any one fighter has one stock left.
    pub is_situation: bool,
}

/// An observer for a scenario where the remaining time is running low.
#[repr(C)]
pub struct StageSituationTime {
    pub base: StageSituationBase,

    /// The maximum required amount of time to put this scenario into effect, in seconds.
    pub remaining_time: f32,

    /// Determines if the battle supports a time limit.
    pub is_valid_time: bool,

    /// Determines if the remaining time is currently counting down.
    pub is_countdown: bool,

    /// Determines if the remaining time has met the threshold.
    pub is_situation: bool,
}

/// An observer for a scenario where one or more fighters have a low amount of stamina.
#[repr(C)]
pub struct StageSituationHP {
    pub base: StageSituationBase,

    /// The maximum required ratio of a fighter's current stamina to their starting stamina to put this scenario into effect.
    pub hp_rate: f32,

    /// Determines if the battle supports stamina.
    pub is_valid_hp: bool,

    /// Determines if any one fighter's stamina has met the threshold.
    pub is_situation: bool,
}

/// An observer for a scenario where the battle is undergoing Sudden Death.
#[repr(C)]
pub struct StageSituationSuddenDeath {
    pub base: StageSituationBase,

    /// Determines if the battle is undergoing Sudden Death.
    pub is_situation: bool,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSituationLink>(), 0x30);
        assert_eq!(offset_of!(StageSituationLink, is_any_situation), 0x0);
        assert_eq!(offset_of!(StageSituationLink, situation), 0x8);
        assert_eq!(offset_of!(StageSituationLink, situation_num), 0x28);

        assert_eq!(size_of::<StageSituationLinkCreateParam>(), 0xC);
        assert_eq!(offset_of!(StageSituationLinkCreateParam, is_link_stock), 0x0);
        assert_eq!(offset_of!(StageSituationLinkCreateParam, is_link_time), 0x1);
        assert_eq!(offset_of!(StageSituationLinkCreateParam, is_link_hp), 0x2);
        assert_eq!(offset_of!(StageSituationLinkCreateParam, remaining_time), 0x4);
        assert_eq!(offset_of!(StageSituationLinkCreateParam, hp_rate), 0x8);

        assert_eq!(size_of::<StageSituationStock>(), 0x10);
        assert_eq!(offset_of!(StageSituationStock, base), 0x0);
        assert_eq!(offset_of!(StageSituationStock, is_valid_stock), 0x8);
        assert_eq!(offset_of!(StageSituationStock, is_situation), 0x9);

        assert_eq!(size_of::<StageSituationTime>(), 0x10);
        assert_eq!(offset_of!(StageSituationTime, base), 0x0);
        assert_eq!(offset_of!(StageSituationTime, remaining_time), 0x8);
        assert_eq!(offset_of!(StageSituationTime, is_valid_time), 0xC);
        assert_eq!(offset_of!(StageSituationTime, is_countdown), 0xD);
        assert_eq!(offset_of!(StageSituationTime, is_situation), 0xE);

        assert_eq!(size_of::<StageSituationHP>(), 0x10);
        assert_eq!(offset_of!(StageSituationHP, base), 0x0);
        assert_eq!(offset_of!(StageSituationHP, hp_rate), 0x8);
        assert_eq!(offset_of!(StageSituationHP, is_valid_hp), 0xC);
        assert_eq!(offset_of!(StageSituationHP, is_situation), 0xD);

        assert_eq!(size_of::<StageSituationSuddenDeath>(), 0x10);
        assert_eq!(offset_of!(StageSituationSuddenDeath, base), 0x0);
        assert_eq!(offset_of!(StageSituationSuddenDeath, is_situation), 0x8);
    }
}
