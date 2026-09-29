/// A color type that expresses its four components as unsigned 8-bit integers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color4u8 {
    inner: [u8; 4],
}

impl Color4u8 {
    /// A color value representing black.
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    /// A color value representing gray.
    pub const GRAY: Self = Self::new(128, 128, 128, 255);

    /// A color value representing white.
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    /// A color value representing red.
    pub const RED: Self = Self::new(255, 0, 0, 255);

    /// A color value representing green.
    pub const GREEN: Self = Self::new(0, 255, 0, 255);

    /// A color value representing blue.
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    /// A color value representing yellow.
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);

    /// A color value representing magenta.
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);

    /// A color value representing cyan.
    pub const CYAN: Self = Self::new(0, 255, 255, 255);

    /// The exponent for converting to sRGB.
    const SRGB_DEFAULT: f32 = Self::LINEAR_DEFAULT.recip();

    /// The exponent for converting to linear color.
    const LINEAR_DEFAULT: f32 = 2.2;

    /// Creates a new `Color4u8`.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: [r, g, b, a],
        }
    }

    /// Returns a reference to the value of the red component.
    pub const fn r(&self) -> &u8 {
        &self.inner[0]
    }

    /// Returns a mutable reference to the value of the red component.
    pub const fn r_mut(&mut self) -> &mut u8 {
        &mut self.inner[0]
    }

    /// Returns a reference to the value of the green component.
    pub const fn g(&self) -> &u8 {
        &self.inner[1]
    }

    /// Returns a mutable reference to the value of the green component.
    pub const fn g_mut(&mut self) -> &mut u8 {
        &mut self.inner[1]
    }

    /// Returns a reference to the value of the blue component.
    pub const fn b(&self) -> &u8 {
        &self.inner[2]
    }

    /// Returns a mutable reference to the value of the blue component.
    pub const fn b_mut(&mut self) -> &mut u8 {
        &mut self.inner[2]
    }

    /// Returns a reference to the value of the alpha component.
    pub const fn a(&self) -> &u8 {
        &self.inner[3]
    }

    /// Returns a mutable reference to the value of the alpha component.
    pub const fn a_mut(&mut self) -> &mut u8 {
        &mut self.inner[3]
    }

    /// Raises the color value to the given exponent.
    pub fn correct_gamma(&self, gamma: f32) -> Self {
        Self::new(
            ((*self.r() as f32 / u8::MAX as f32)
                .powf(gamma)
                .clamp(0.0, 1.0)
                * u8::MAX as f32) as u8,
            ((*self.g() as f32 / u8::MAX as f32)
                .powf(gamma)
                .clamp(0.0, 1.0)
                * u8::MAX as f32) as u8,
            ((*self.b() as f32 / u8::MAX as f32)
                .powf(gamma)
                .clamp(0.0, 1.0)
                * u8::MAX as f32) as u8,
            *self.a(),
        )
    }

    /// Converts the given color to sRGB.
    pub fn to_srgb(&self) -> Self {
        self.correct_gamma(Self::SRGB_DEFAULT)
    }

    /// Converts the given color to linear color.
    pub fn to_linear(&self) -> Self {
        self.correct_gamma(Self::LINEAR_DEFAULT)
    }

    /// Returns `true` if the alpha component is completely transparent.
    pub const fn is_transparent(&self) -> bool {
        *self.a() == 0
    }
}

/// A color type that expresses its four components as 32-bit floating-point numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, align(16))]
pub struct Color4f {
    inner: [f32; 4],
}

impl Color4f {
    /// A color value representing black.
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// A color value representing gray.
    pub const GRAY: Self = Self::new(0.5, 0.5, 0.5, 1.0);

    /// A color value representing white.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// A color value representing red.
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);

    /// A color value representing green.
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);

    /// A color value representing blue.
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    /// A color value representing yellow.
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);

    /// A color value representing magenta.
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    /// A color value representing cyan.
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);

    /// The exponent for converting to sRGB.
    const SRGB_DEFAULT: f32 = Self::LINEAR_DEFAULT.recip();

    /// The exponent for converting to linear color.
    const LINEAR_DEFAULT: f32 = 2.2;

    /// Creates a new `Color4f`.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: [r, g, b, a],
        }
    }

    /// Returns a reference to the value of the red component.
    pub const fn r(&self) -> &f32 {
        &self.inner[0]
    }

    /// Returns a mutable reference to the value of the red component.
    pub const fn r_mut(&mut self) -> &mut f32 {
        &mut self.inner[0]
    }

    /// Returns a reference to the value of the green component.
    pub const fn g(&self) -> &f32 {
        &self.inner[1]
    }

    /// Returns a mutable reference to the value of the green component.
    pub const fn g_mut(&mut self) -> &mut f32 {
        &mut self.inner[1]
    }

    /// Returns a reference to the value of the blue component.
    pub const fn b(&self) -> &f32 {
        &self.inner[2]
    }

    /// Returns a mutable reference to the value of the blue component.
    pub const fn b_mut(&mut self) -> &mut f32 {
        &mut self.inner[2]
    }

    /// Returns a reference to the value of the alpha component.
    pub const fn a(&self) -> &f32 {
        &self.inner[3]
    }

    /// Returns a reference to the value of the alpha component.
    pub const fn a_mut(&mut self) -> &mut f32 {
        &mut self.inner[3]
    }

    /// Raises the color value to the given exponent.
    pub fn correct_gamma(&self, gamma: f32) -> Self {
        Self::new(
            self.r().powf(gamma),
            self.g().powf(gamma),
            self.b().powf(gamma),
            *self.a(),
        )
    }

    /// Converts the given color to sRGB.
    pub fn to_srgb(&self) -> Self {
        self.correct_gamma(Self::SRGB_DEFAULT)
    }

    /// Converts the given color to linear color.
    pub fn to_linear(&self) -> Self {
        self.correct_gamma(Self::LINEAR_DEFAULT)
    }

    /// Returns `true` if the alpha component is completely transparent.
    pub const fn is_transparent(&self) -> bool {
        *self.a() <= 0.0
    }
}
