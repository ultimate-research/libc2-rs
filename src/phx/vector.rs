use core::{fmt, ops};

/// A two-dimensional vector type.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C, align(8))]
pub struct Vector2f {
    inner: [f32; 2],
}

impl Vector2f {
    /// A zero vector.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// A vector of all ones.
    pub const ONE: Self = Self::new(1.0, 1.0);

    /// A unit vector representing the x-axis.
    pub const UNIT_X: Self = Self::new(1.0, 0.0);

    /// A unit vector representing the y-axis.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0);

    /// Creates a new `Vector2f`.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { inner: [x, y] }
    }

    /// Returns a reference to the component along the x-axis.
    pub const fn x(&self) -> &f32 {
        &self.inner[0]
    }

    /// Returns a mutable reference to the component along the x-axis.
    pub const fn x_mut(&mut self) -> &mut f32 {
        &mut self.inner[0]
    }

    /// Returns a reference to the component along the y-axis.
    pub const fn y(&self) -> &f32 {
        &self.inner[1]
    }

    /// Returns a mutable reference to the component along the y-axis.
    pub const fn y_mut(&mut self) -> &mut f32 {
        &mut self.inner[1]
    }

    /// Computes the normalization of `self`.
    pub fn normalize(self) -> Self {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            self / length
        } else {
            Self::ZERO
        }
    }

    /// Normalizes `self`, returning `true` if successful.
    pub fn normalize_mut(&mut self) -> bool {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            *self /= length;
        } else {
            *self = Self::ZERO;
        }

        length_squared != 0.0
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y()
    }

    /// Computes the magnitude of the cross product of two vectors.
    pub fn cross(&self, other: &Self) -> f32 {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Computes the length of `self`.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Computes the squared length of `self`.
    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y()
    }

    /// Computes the distance between two points.
    pub fn distance(a: Self, b: Self) -> f32 {
        Self::distance_squared(a, b).sqrt()
    }

    /// Computes the squared distance between two points.
    pub fn distance_squared(a: Self, b: Self) -> f32 {
        (a - b).length_squared()
    }

    /// Returns a vector consisting of the minimum value of each component of two vectors.
    pub const fn min(&self, other: &Self) -> Self {
        Self::new(self.x().min(*other.x()), self.y().min(*other.y()))
    }

    /// Returns a vector consisting of the maximum value of each component of two vectors.
    pub const fn max(&self, other: &Self) -> Self {
        Self::new(self.x().max(*other.x()), self.y().max(*other.y()))
    }

    /// Computes the result of linearly interpolating between two points.
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl fmt::Display for Vector2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.x(), self.y())
    }
}

impl ops::Neg for Vector2f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y())
    }
}

impl ops::Add for Vector2f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }
}

impl ops::AddAssign for Vector2f {
    fn add_assign(&mut self, other: Self) {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
    }
}

impl ops::Sub for Vector2f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }
}

impl ops::SubAssign for Vector2f {
    fn sub_assign(&mut self, other: Self) {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
    }
}

impl ops::Mul<f32> for Vector2f {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(self.x() * other, self.y() * other)
    }
}

impl ops::MulAssign<f32> for Vector2f {
    fn mul_assign(&mut self, other: f32) {
        *self.x_mut() *= other;
        *self.y_mut() *= other;
    }
}

impl ops::Div<f32> for Vector2f {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::new(self.x() / other, self.y() / other)
    }
}

impl ops::DivAssign<f32> for Vector2f {
    fn div_assign(&mut self, other: f32) {
        *self.x_mut() /= other;
        *self.y_mut() /= other;
    }
}

/// A three-dimensional vector type.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C, align(16))]
pub struct Vector3f {
    inner: [f32; 3],
}

impl Vector3f {
    /// A zero vector.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// A vector of all ones.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    /// A unit vector representing the x-axis.
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0);

    /// A unit vector representing the y-axis.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0);

    /// A unit vector representing the z-axis.
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0);

    /// Creates a new `Vector3f`.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { inner: [x, y, z] }
    }

    /// Returns a reference to the component along the x-axis.
    pub const fn x(&self) -> &f32 {
        &self.inner[0]
    }

    /// Returns a mutable reference to the component along the x-axis.
    pub const fn x_mut(&mut self) -> &mut f32 {
        &mut self.inner[0]
    }

    /// Returns a reference to the component along the y-axis.
    pub const fn y(&self) -> &f32 {
        &self.inner[1]
    }

    /// Returns a mutable reference to the component along the y-axis.
    pub const fn y_mut(&mut self) -> &mut f32 {
        &mut self.inner[1]
    }

    /// Returns a reference to the component along the z-axis.
    pub const fn z(&self) -> &f32 {
        &self.inner[2]
    }

    /// Returns a mutable reference to the component along the z-axis.
    pub const fn z_mut(&mut self) -> &mut f32 {
        &mut self.inner[2]
    }

    /// Computes the normalization of `self`.
    pub fn normalize(self) -> Self {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            self / length
        } else {
            Self::ZERO
        }
    }

    /// Normalizes `self`, returning `true` if successful.
    pub fn normalize_mut(&mut self) -> bool {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            *self /= length;
        } else {
            *self = Self::ZERO;
        }

        length_squared != 0.0
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Computes the cross product of two vectors.
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    /// Computes the length of `self`.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Computes the squared length of `self`.
    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    /// Computes the distance between two points.
    pub fn distance(a: Self, b: Self) -> f32 {
        Self::distance_squared(a, b).sqrt()
    }

    /// Computes the squared distance between two points.
    pub fn distance_squared(a: Self, b: Self) -> f32 {
        (a - b).length_squared()
    }

    /// Returns a vector consisting of the minimum value of each component of two vectors.
    pub const fn min(&self, other: &Self) -> Self {
        Self::new(
            self.x().min(*other.x()),
            self.y().min(*other.y()),
            self.z().min(*other.z()),
        )
    }

    /// Returns a vector consisting of the maximum value of each component of two vectors.
    pub const fn max(&self, other: &Self) -> Self {
        Self::new(
            self.x().max(*other.x()),
            self.y().max(*other.y()),
            self.z().max(*other.z()),
        )
    }

    /// Computes the result of linearly interpolating between two points.
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl fmt::Display for Vector3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x(), self.y(), self.z())
    }
}

impl ops::Neg for Vector3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vector3f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::AddAssign for Vector3f {
    fn add_assign(&mut self, other: Self) {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
        *self.z_mut() += other.z();
    }
}

impl ops::Sub for Vector3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::SubAssign for Vector3f {
    fn sub_assign(&mut self, other: Self) {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
        *self.z_mut() -= other.z();
    }
}

impl ops::Mul<f32> for Vector3f {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::MulAssign<f32> for Vector3f {
    fn mul_assign(&mut self, other: f32) {
        *self.x_mut() *= other;
        *self.y_mut() *= other;
        *self.z_mut() *= other;
    }
}

impl ops::Div<f32> for Vector3f {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl ops::DivAssign<f32> for Vector3f {
    fn div_assign(&mut self, other: f32) {
        *self.x_mut() /= other;
        *self.y_mut() /= other;
        *self.z_mut() /= other;
    }
}

/// A four-dimensional vector type.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C, align(16))]
pub struct Vector4f {
    inner: [f32; 4],
}

impl Vector4f {
    /// A zero vector.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// A vector of all ones.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// A unit vector representing the x-axis.
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// A unit vector representing the y-axis.
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// A unit vector representing the z-axis.
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// A unit vector representing the w-axis.
    pub const UNIT_W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Creates a new `Vector4f`.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            inner: [x, y, z, w],
        }
    }

    /// Returns a reference to the component along the x-axis.
    pub const fn x(&self) -> &f32 {
        &self.inner[0]
    }

    /// Returns a mutable reference to the component along the x-axis.
    pub const fn x_mut(&mut self) -> &mut f32 {
        &mut self.inner[0]
    }

    /// Returns a reference to the component along the y-axis.
    pub const fn y(&self) -> &f32 {
        &self.inner[1]
    }

    /// Returns a mutable reference to the component along the y-axis.
    pub const fn y_mut(&mut self) -> &mut f32 {
        &mut self.inner[1]
    }

    /// Returns a reference to the component along the z-axis.
    pub const fn z(&self) -> &f32 {
        &self.inner[2]
    }

    /// Returns a mutable reference to the component along the z-axis.
    pub const fn z_mut(&mut self) -> &mut f32 {
        &mut self.inner[2]
    }

    /// Returns a reference to the component along the w-axis.
    pub const fn w(&self) -> &f32 {
        &self.inner[3]
    }

    /// Returns a mutable reference to the component along the w-axis.
    pub const fn w_mut(&mut self) -> &mut f32 {
        &mut self.inner[3]
    }

    /// Computes the normalization of `self`.
    pub fn normalize(self) -> Self {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            self / length
        } else {
            Self::ZERO
        }
    }

    /// Normalizes `self`, returning `true` if successful.
    pub fn normalize_mut(&mut self) -> bool {
        let length_squared = self.length_squared();

        if length_squared > 0.0 {
            let length = length_squared.sqrt();

            *self /= length;
        } else {
            *self = Self::ZERO;
        }

        length_squared != 0.0
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    /// Computes the length of `self`.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Computes the squared length of `self`.
    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    /// Computes the distance between two points.
    pub fn distance(a: Self, b: Self) -> f32 {
        Self::distance_squared(a, b).sqrt()
    }

    /// Computes the squared distance between two points.
    pub fn distance_squared(a: Self, b: Self) -> f32 {
        (a - b).length_squared()
    }

    /// Returns a vector consisting of the minimum value of each component of two vectors.
    pub fn min(a: &Self, b: &Self) -> Self {
        Self::new(
            a.x().min(*b.x()),
            a.y().min(*b.y()),
            a.z().min(*b.z()),
            a.w().min(*b.w()),
        )
    }

    /// Returns a vector consisting of the maximum value of each component of two vectors.
    pub fn max(a: &Self, b: &Self) -> Self {
        Self::new(
            a.x().max(*b.x()),
            a.y().max(*b.y()),
            a.z().max(*b.z()),
            a.w().max(*b.w()),
        )
    }

    /// Computes the result of linearly interpolating between two points.
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl fmt::Display for Vector4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{}, {}, {}, {}>",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

impl ops::Neg for Vector4f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl ops::Add for Vector4f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
            self.w() + other.w(),
        )
    }
}

impl ops::AddAssign for Vector4f {
    fn add_assign(&mut self, other: Self) {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
        *self.z_mut() += other.z();
        *self.w_mut() += other.w();
    }
}

impl ops::Sub for Vector4f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
            self.w() - other.w(),
        )
    }
}

impl ops::SubAssign for Vector4f {
    fn sub_assign(&mut self, other: Self) {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
        *self.z_mut() -= other.z();
        *self.w_mut() -= other.w();
    }
}

impl ops::Mul<f32> for Vector4f {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(
            self.x() * other,
            self.y() * other,
            self.z() * other,
            self.w() * other,
        )
    }
}

impl ops::MulAssign<f32> for Vector4f {
    fn mul_assign(&mut self, other: f32) {
        *self.x_mut() *= other;
        *self.y_mut() *= other;
        *self.z_mut() *= other;
        *self.w_mut() *= other;
    }
}

impl ops::Div<f32> for Vector4f {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::new(
            self.x() / other,
            self.y() / other,
            self.z() / other,
            self.w() / other,
        )
    }
}

impl ops::DivAssign<f32> for Vector4f {
    fn div_assign(&mut self, other: f32) {
        *self.x_mut() /= other;
        *self.y_mut() /= other;
        *self.z_mut() /= other;
        *self.w_mut() /= other;
    }
}
