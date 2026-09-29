#[repr(C)]
pub struct StageRandom {
    inner: [u32; 4],
}

impl StageRandom {
    /// Creates a new `StageRandom`.
    pub fn new(mut seed: u32) -> Self {
        Self {
            inner: core::array::from_fn(|i| {
                seed = (seed ^ seed >> 30)
                    .wrapping_mul(0x6C078965)
                    .wrapping_add(i as u32);
                seed
            }),
        }
    }

    /// Computes the next psuedorandom number.
    #[inline(always)]
    fn get(&mut self) -> u32 {
        let mut no = self.inner[0];

        no ^= no << 11;
        no ^= no >> 8;
        no ^= self.inner[3];
        no ^= self.inner[3] >> 19;
        self.inner[0] = self.inner[1];
        self.inner[1] = self.inner[2];
        self.inner[2] = self.inner[3];
        self.inner[3] = no;

        no
    }

    /// Returns `true` if the next psuedorandom number is odd.
    #[inline(always)]
    pub fn next_bool(&mut self) -> bool {
        self.get() & 1 != 0
    }

    /// Computes the next psuedorandom `u32`.
    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        self.get()
    }

    /// Computes the next psuedorandom `f32` within the interval `0.0..1.0`.
    #[inline(always)]
    pub fn next_f32(&mut self) -> f32 {
        self.get() as f32 * (1.0 / u32::MAX as f32)
    }

    /// Computes the next psuedorandom `u32` within the interval `min..=max`.
    #[inline(always)]
    pub fn range_u32(&mut self, min: u32, max: u32) -> u32 {
        min + self.next_u32() % 1u32.wrapping_sub(min).wrapping_add(max)
    }

    /// Computes the next psuedorandom `f32` within the interval `min..max`.
    #[inline(always)]
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

impl Default for StageRandom {
    fn default() -> Self {
        Self {
            inner: [0x15CD5B07, 0xE5559A15, 0xB53B121F, 0x33134905],
        }
    }
}
