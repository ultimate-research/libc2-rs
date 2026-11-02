#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The set of configured options for the guest characters.
    pub guest_info: GuestInfo,

    /// The selected index into `scene_start_frame`.
    pub scene_start_frame_index: u8,
}

/// A set of configurable options for the guest characters.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GuestInfo {
    inner: u8,
}

impl GuestInfo {
    /// The bitmask for the selected pair of guest characters.
    const GUEST_MASK: u8 = 0b00001111;

    /// The bitmask for the selected array index into `positions`.
    const POSITION_MASK: u8 = 0b11110000;

    /// The right-hand shift operand for extracting the selected array index into `positions`.
    const POSITION_SHIFT: u32 = Self::POSITION_MASK.trailing_zeros();

    /// Creates a new `GuestInfo`.
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Creates a new `GuestInfo` with the given pair of guest characters.
    pub const fn with_guest(mut self, guest: Guest) -> Self {
        self.set_guest(guest);
        self
    }

    /// Creates a new `GuestInfo` with the given array index into `positions`.
    pub const fn with_position(mut self, index: usize) -> Self {
        self.set_position(index);
        self
    }

    /// Extracts the selected pair of guest characters.
    pub const fn guest(&self) -> Option<Guest> {
        match self.inner & Self::GUEST_MASK {
            0 => Some(Guest::MelephKagutsuchi),
            1 => Some(Guest::NiyahByakko),
            2 => Some(Guest::ToraHanaJS),
            3 => Some(Guest::ZekeSaika),
            _ => None,
        }
    }

    /// Extracts the selected array index into `positions`.
    pub const fn position(&self) -> usize {
        ((self.inner & Self::POSITION_MASK) >> Self::POSITION_SHIFT) as usize
    }

    /// Assigns the given pair of guest characters to the selected pair of guest characters.
    pub const fn set_guest(&mut self, guest: Guest) {
        let guest = guest as u8 & Self::GUEST_MASK;

        self.inner = (self.inner & !Self::GUEST_MASK) | guest;
    }

    /// Assigns the given array index to the selected array index into `positions`.
    pub const fn set_position(&mut self, index: usize) {
        let index = ((index as u8) << Self::POSITION_SHIFT) & Self::POSITION_MASK;

        self.inner = (self.inner & !Self::POSITION_MASK) | index;
    }

    /// Consumes the `GuestInfo`, returning the underlying value.
    pub const fn into_inner(self) -> u8 {
        self.inner
    }
}

/// A numeric identifier for a pair of guest characters.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Guest {
    /// Mòrag and Brighid
    #[default]
    MelephKagutsuchi,

    /// Nia and Dromarch
    NiyahByakko,

    /// Tora and Poppi α
    ToraHanaJS,

    /// Zeke and Pandoria
    ZekeSaika,
}
