#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct RandomSetting {
    /// The selected pet to appear in the background.
    pub pet: Pet,

    /// The combination of selectable routes for the puppy.
    pub dog_route: DogRoute,

    /// The selected route for the kitten.
    pub cat_route: u8,
}

/// A numeric identifier for a pet.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Pet {
    /// Beagle
    #[default]
    DogB,

    /// Toy Poodle
    DogP,

    /// Shiba Inu
    DogS,

    /// Jack Russell Terrier
    DogJ,

    /// Golden Retriever
    DogG,

    /// Calico Cat
    Cat,
}

/// A combination of selectable routes for the puppy.
///
/// Each of the four nybbles corresponds to a specific section, in order:
///
/// - `out_to_a`; the entrance
/// - `a_to_b`; the first intermediary behavior
/// - `b_to_c`; the second intermediary behavior
/// - `c_to_out`; the exit
///
/// Upon each loop of the complete route, another route in the section may be selected.
///
/// At least one bit in each nybble must be set, otherwise an infinite loop
/// will occur, as the game will indefinitely search for a nonzero bit in a
/// copy of the bit field.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct DogRoute {
    inner: u16,
}

impl DogRoute {
    /// Creates a new `DogRoute`.
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Creates a new `DogRoute` with the given route available for selection.
    pub const fn with_route(mut self, id: u32) -> Self {
        self.on_route(id);
        self
    }

    /// Returns `true` if the given route is selectable.
    pub const fn is_route(&self, id: u32) -> bool {
        self.inner & (1 << id) != 0
    }

    /// Enables the selectability of the given route.
    pub const fn on_route(&mut self, id: u32) {
        self.inner |= 1 << id;
    }

    /// Disables the selectability of the given route.
    pub const fn off_route(&mut self, id: u32) {
        self.inner &= !(1 << id);
    }

    /// Consumes the `DogRoute`, returning the underlying value.
    pub const fn into_inner(self) -> u16 {
        self.inner
    }
}
