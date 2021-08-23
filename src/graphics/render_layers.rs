use kecs::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Component)]
/// Used to configure which layers [Entity]s will render on.
pub struct RenderLayers(usize);

impl RenderLayers {
    pub const NONE: RenderLayers = RenderLayers(0);
    pub const DEFAULT: RenderLayers = RenderLayers(1 << 0);
    pub const ONE: RenderLayers = RenderLayers(1 << 1);
    pub const TWO: RenderLayers = RenderLayers(1 << 2);
    pub const THREE: RenderLayers = RenderLayers(1 << 3);
    pub const USER_INTERFACE: RenderLayers = RenderLayers(1 << 8);

    pub fn enable_layers(&mut self, layer: RenderLayers) {
        self.0 |= layer.0
    }

    pub fn disable_layers(&mut self, layer: RenderLayers) {
        self.0 &= !layer.0
    }

    /// This will return true if *any* of the other `RenderLayer`'s layers are
    /// included in this layer.
    pub fn includes_layer(&self, layer: RenderLayers) -> bool {
        self.0 & layer.0 != 0
    }
}
