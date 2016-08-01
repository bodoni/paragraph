//! Line-breaking algorithm.

/// An item.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    /// The type.
    pub t: Type,
    /// The width.
    pub w: f32,
    /// The stretchability.
    pub y: f32,
    /// The shrinkability.
    pub z: f32,
    /// The penalty.
    pub p: f32,
    /// The flag.
    pub f: bool,
}

/// An item type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// A box item.
    Box,
    /// A glue item.
    Glue,
    /// A penalty item.
    Penalty,
}

impl Item {
    /// Create an item.
    #[inline]
    pub fn new(t: Type, w: f32, y: f32, z: f32, p: f32, f: bool) -> Item {
        Item { t: t, w: w, y: y, z: z, p: p, f: f }
    }

    /// Create a box item.
    #[inline]
    pub fn new_box(w: f32) -> Item {
        Item::new(Type::Box, w, 0.0, 0.0, 0.0, false)
    }

    /// Create a glue item.
    #[inline]
    pub fn new_glue(w: f32, y: f32, z: f32) -> Item {
        Item::new(Type::Glue, w, y, z, 0.0, false)
    }

    /// Create a penalty item.
    #[inline]
    pub fn new_penalty(w: f32, p: f32, f: bool) -> Item {
        Item::new(Type::Penalty, w, 0.0, 0.0, p, f)
    }

    /// Check if it is a box item.
    #[inline]
    pub fn is_box(&self) -> bool {
        self.t == Type::Box
    }

    /// Check if it is a glue item.
    #[inline]
    pub fn is_glue(&self) -> bool {
        self.t == Type::Glue
    }

    /// Check if it is a penalty item.
    #[inline]
    pub fn is_penalty(&self) -> bool {
        self.t == Type::Penalty
    }
}
