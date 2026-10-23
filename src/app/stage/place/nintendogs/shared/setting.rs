use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct RandomSetting {
    /// The selected pet to appear in the background.
    pub pet: Pet,

    /// The selected route segments for the puppy.
    pub dog_route: DogRoute,

    /// The selected route for the kitten.
    pub cat_route: u8,
}

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

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct DogRoute {
    inner: u16,
}

impl DogRoute {
    /// The bitmask for the raw value corresponding to the entrance route segment.
    const OUT_TO_A_MASK: u16 = 0b0000_0000_0000_1111;

    /// The right-hand shift operand for the raw value corresponding to the entrance route segment.
    const OUT_TO_A_SHIFT: u32 = Self::OUT_TO_A_MASK.leading_zeros();

    /// The bitmask for the raw value corresponding to the first intermediate route segment.
    const A_TO_B_MASK: u16 = 0b0000_0000_1111_0000;

    /// The right-hand shift operand for the raw value corresponding to the first intermediate route segment.
    const A_TO_B_SHIFT: u32 = Self::A_TO_B_MASK.leading_zeros();

    /// The bitmask for the raw value corresponding to the second intermediate route segment.
    const B_TO_C_MASK: u16 = 0b0000_1111_0000_0000;

    /// The right-hand shift operand for the raw value corresponding to the second intermediate route segment.
    const B_TO_C_SHIFT: u32 = Self::B_TO_C_MASK.leading_zeros();

    /// The bitmask for the raw value corresponding to the exit route segment.
    const C_TO_OUT_MASK: u16 = 0b1111_0000_0000_0000;

    /// The right-hand shift operand for the raw value corresponding to the exit route segment.
    const C_TO_OUT_SHIFT: u32 = Self::C_TO_OUT_MASK.leading_zeros();

    /// Creates a new `DogRoute` from the raw values of each route segment.
    pub const fn new(
        out_to_a: NonZeroU16,
        a_to_b: NonZeroU16,
        b_to_c: NonZeroU16,
        c_to_out: NonZeroU16,
    ) -> Option<Self> {
        let out_to_a = if out_to_a.get() < 16 {
            out_to_a.get() << Self::OUT_TO_A_SHIFT & Self::OUT_TO_A_MASK
        } else {
            return None;
        };

        let a_to_b = if a_to_b.get() < 16 {
            a_to_b.get() << Self::A_TO_B_SHIFT & Self::A_TO_B_MASK
        } else {
            return None;
        };

        let b_to_c = if b_to_c.get() < 16 {
            b_to_c.get() << Self::B_TO_C_SHIFT & Self::B_TO_C_MASK
        } else {
            return None;
        };

        let c_to_out = if c_to_out.get() < 16 {
            c_to_out.get() << Self::C_TO_OUT_SHIFT & Self::C_TO_OUT_MASK
        } else {
            return None;
        };

        Some(Self {
            inner: out_to_a | a_to_b | b_to_c | c_to_out,
        })
    }

    /// Extracts the raw value corresponding to the entrance route segment.
    pub const fn out_to_a(&self) -> Option<NonZeroU16> {
        let route = (self.inner & Self::OUT_TO_A_MASK) >> Self::OUT_TO_A_SHIFT;

        NonZeroU16::new(route)
    }

    /// Extracts the raw value corresponding to the first intermediate route segment.
    pub const fn a_to_b(&self) -> Option<NonZeroU16> {
        let route = (self.inner & Self::A_TO_B_MASK) >> Self::A_TO_B_SHIFT;

        NonZeroU16::new(route)
    }

    /// Extracts the raw value corresponding to the second intermediate route segment.
    pub const fn b_to_c(&self) -> Option<NonZeroU16> {
        let route = (self.inner & Self::B_TO_C_MASK) >> Self::B_TO_C_SHIFT;

        NonZeroU16::new(route)
    }

    /// Extracts the raw value corresponding to the exit route segment.
    pub const fn c_to_out(&self) -> Option<NonZeroU16> {
        let route = (self.inner & Self::C_TO_OUT_MASK) >> Self::C_TO_OUT_SHIFT;

        NonZeroU16::new(route)
    }
}
