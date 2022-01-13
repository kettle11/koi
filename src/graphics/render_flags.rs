use kecs::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Component)]
/// Used to configure which layers [Entity]s will render on.
pub struct RenderFlags(usize);

impl RenderFlags {
    pub const NONE: RenderFlags = RenderFlags(0);
    pub const DEFAULT: RenderFlags = RenderFlags(1 << 0);
    pub const DO_NOT_CAST_SHADOWS: RenderFlags = RenderFlags(1 << 1);
    pub const IGNORE_CULLING: RenderFlags = RenderFlags(1 << 2);
    pub const USER_INTERFACE: RenderFlags = RenderFlags(1 << 8);

    pub const fn with_layer(mut self, layer: RenderFlags) -> Self {
        self.0 |= layer.0;
        self
    }

    /// This will return true if *any* of the other `RenderLayer`'s layers are
    /// included in this layer.
    pub const fn includes_layer(&self, layer: RenderFlags) -> bool {
        self.0 & layer.0 != 0
    }
}
