/// A floating-point rectangle type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rect {
    /// The coordinate of the left edge.
    left: f32,

    /// The coordinate of the right edge.
    right: f32,

    /// The coordinate of the top edge.
    top: f32,

    /// The coordinate of the bottom edge.
    bottom: f32,
}

impl Rect {
    /// Creates a new `Rect`.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Returns a reference to the coordinate of the left edge.
    pub const fn left(&self) -> &f32 {
        &self.left
    }

    /// Returns a mutable reference to the coordinate of the left edge.
    pub const fn left_mut(&mut self) -> &mut f32 {
        &mut self.left
    }

    /// Returns a reference to the coordinate of the right edge.
    pub const fn right(&self) -> &f32 {
        &self.right
    }

    /// Returns a mutable reference to the coordinate of the right edge.
    pub const fn right_mut(&mut self) -> &mut f32 {
        &mut self.right
    }

    /// Returns a reference to the coordinate of the top edge.
    pub const fn top(&self) -> &f32 {
        &self.top
    }

    /// Returns a mutable reference to the coordinate of the top edge.
    pub const fn top_mut(&mut self) -> &mut f32 {
        &mut self.top
    }

    /// Returns a reference to the coordinate of the bottom edge.
    pub const fn bottom(&self) -> &f32 {
        &self.bottom
    }

    /// Returns a mutable reference to the coordinate of the bottom edge.
    pub const fn bottom_mut(&mut self) -> &mut f32 {
        &mut self.bottom
    }

    /// Computes the area of the rectangle.
    pub fn area(&self) -> f32 {
        (self.right() - self.left()) * (self.top() - self.bottom())
    }
}
