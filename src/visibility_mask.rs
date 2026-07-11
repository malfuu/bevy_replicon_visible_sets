use bevy::prelude::*;

/// Not to be confused with bevy_replicon's own visibility mask,
/// and corresponding [`bevy_replicon::server::visibility::filters_mask::FilterBit`]s.
#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Clone, Copy)]
pub struct VisibilityMask(pub u32);

impl VisibilityMask {
    /// Contains all layers.
    pub const ALL: Self = Self(0xffff_ffff);
    /// Contains no layers.
    pub const NONE: Self = Self(0);
}
