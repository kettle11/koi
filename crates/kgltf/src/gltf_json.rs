use kserde::*;

use std::collections::HashMap;

/// The root object for a glTF asset.
#[derive(Debug, Clone)]
pub struct GlTf {
    /// Names of glTF extensions used in this asset.
    pub extensions_used: Vec<String>,
    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Vec<String>,
    /// An array of accessors.
    pub accessors: Vec<Accessor>,
    /// An array of keyframe animations.
    pub animations: Vec<Animation>,
    /// Metadata about the glTF asset.
    pub asset: Asset,
    /// An array of buffers.
    pub buffers: Vec<Buffer>,
    /// An array of bufferViews.
    pub buffer_views: Vec<BufferView>,
    /// An array of cameras.
    pub cameras: Vec<Camera>,
    /// An array of images.
    pub images: Vec<Image>,
    /// An array of materials.
    pub materials: Vec<Material>,
    /// An array of meshes.
    pub meshes: Vec<Mesh>,
    /// An array of nodes.
    pub nodes: Vec<Node>,
    /// An array of samplers.
    pub samplers: Vec<Sampler>,
    /// The index of the default scene.
    pub scene: Option<usize>,
    /// An array of scenes.
    pub scenes: Vec<Scene>,
    /// An array of skins.
    pub skins: Vec<Skin>,
    /// An array of textures.
    pub textures: Vec<Texture>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for GlTf {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if !self.extensions_used.is_empty() {
            serializer.property("extensionsUsed");
            serializer.value(&self.extensions_used);
        }
        if !self.extensions_required.is_empty() {
            serializer.property("extensionsRequired");
            serializer.value(&self.extensions_required);
        }
        if !self.accessors.is_empty() {
            serializer.property("accessors");
            serializer.value(&self.accessors);
        }
        if !self.animations.is_empty() {
            serializer.property("animations");
            serializer.value(&self.animations);
        }
        serializer.property("asset");
        serializer.value(&self.asset);
        if !self.buffers.is_empty() {
            serializer.property("buffers");
            serializer.value(&self.buffers);
        }
        if !self.buffer_views.is_empty() {
            serializer.property("bufferViews");
            serializer.value(&self.buffer_views);
        }
        if !self.cameras.is_empty() {
            serializer.property("cameras");
            serializer.value(&self.cameras);
        }
        if !self.images.is_empty() {
            serializer.property("images");
            serializer.value(&self.images);
        }
        if !self.materials.is_empty() {
            serializer.property("materials");
            serializer.value(&self.materials);
        }
        if !self.meshes.is_empty() {
            serializer.property("meshes");
            serializer.value(&self.meshes);
        }
        if !self.nodes.is_empty() {
            serializer.property("nodes");
            serializer.value(&self.nodes);
        }
        if !self.samplers.is_empty() {
            serializer.property("samplers");
            serializer.value(&self.samplers);
        }
        if let Some(v) = self.scene.as_ref() {
            serializer.property("scene");
            serializer.value(&v);
        }
        if !self.scenes.is_empty() {
            serializer.property("scenes");
            serializer.value(&self.scenes);
        }
        if !self.skins.is_empty() {
            serializer.property("skins");
            serializer.value(&self.skins);
        }
        if !self.textures.is_empty() {
            serializer.property("textures");
            serializer.value(&self.textures);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for GlTf {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut extensions_used = None;
        let mut extensions_required = None;
        let mut accessors = None;
        let mut animations = None;
        let mut asset = None;
        let mut buffers = None;
        let mut buffer_views = None;
        let mut cameras = None;
        let mut images = None;
        let mut materials = None;
        let mut meshes = None;
        let mut nodes = None;
        let mut samplers = None;
        let mut scene = None;
        let mut scenes = None;
        let mut skins = None;
        let mut textures = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            println!("PROPERTY: {:?}", property);
            match &*property {
                "extensionsUsed" => {
                    extensions_used = Some(<Vec<String>>::deserialize(deserializer)?)
                }
                "extensionsRequired" => {
                    extensions_required = Some(<Vec<String>>::deserialize(deserializer)?)
                }
                "accessors" => accessors = Some(<Vec<Accessor>>::deserialize(deserializer)?),
                "animations" => animations = Some(<Vec<Animation>>::deserialize(deserializer)?),
                "asset" => asset = Some(<Asset>::deserialize(deserializer)?),
                "buffers" => buffers = Some(<Vec<Buffer>>::deserialize(deserializer)?),
                "bufferViews" => buffer_views = Some(<Vec<BufferView>>::deserialize(deserializer)?),
                "cameras" => cameras = Some(<Vec<Camera>>::deserialize(deserializer)?),
                "images" => images = Some(<Vec<Image>>::deserialize(deserializer)?),
                "materials" => materials = Some(<Vec<Material>>::deserialize(deserializer)?),
                "meshes" => meshes = Some(<Vec<Mesh>>::deserialize(deserializer)?),
                "nodes" => nodes = Some(<Vec<Node>>::deserialize(deserializer)?),
                "samplers" => samplers = Some(<Vec<Sampler>>::deserialize(deserializer)?),
                "scene" => scene = Some(<usize>::deserialize(deserializer)?),
                "scenes" => scenes = Some(<Vec<Scene>>::deserialize(deserializer)?),
                "skins" => skins = Some(<Vec<Skin>>::deserialize(deserializer)?),
                "textures" => textures = Some(<Vec<Texture>>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            extensions_used: extensions_used.unwrap_or_else(Vec::new),
            extensions_required: extensions_required.unwrap_or_else(Vec::new),
            accessors: accessors.unwrap_or_else(Vec::new),
            animations: animations.unwrap_or_else(Vec::new),
            asset: asset?,
            buffers: buffers.unwrap_or_else(Vec::new),
            buffer_views: buffer_views.unwrap_or_else(Vec::new),
            cameras: cameras.unwrap_or_else(Vec::new),
            images: images.unwrap_or_else(Vec::new),
            materials: materials.unwrap_or_else(Vec::new),
            meshes: meshes.unwrap_or_else(Vec::new),
            nodes: nodes.unwrap_or_else(Vec::new),
            samplers: samplers.unwrap_or_else(Vec::new),
            scene: scene,
            scenes: scenes.unwrap_or_else(Vec::new),
            skins: skins.unwrap_or_else(Vec::new),
            textures: textures.unwrap_or_else(Vec::new),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// A texture and its sampler.
#[derive(Debug, Clone)]
pub struct Texture {
    /// The index of the sampler used by this texture. When undefined, a sampler with repeat wrapping and auto filtering **SHOULD** be used.
    pub sampler: Option<usize>,
    /// The index of the image used by this texture. When undefined, an extension or other mechanism **SHOULD** supply an alternate texture source, otherwise behavior is undefined.
    pub source: Option<usize>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Texture {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.sampler.as_ref() {
            serializer.property("sampler");
            serializer.value(&v);
        }
        if let Some(v) = self.source.as_ref() {
            serializer.property("source");
            serializer.value(&v);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Texture {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut sampler = None;
        let mut source = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "sampler" => sampler = Some(<usize>::deserialize(deserializer)?),
                "source" => source = Some(<usize>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            sampler: sampler,
            source: source,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Joints and matrices defining a skin.
#[derive(Debug, Clone)]
pub struct Skin {
    /// The index of the accessor containing the floating-point 4x4 inverse-bind matrices.
    pub inverse_bind_matrices: Option<usize>,
    /// The index of the node used as a skeleton root.
    pub skeleton: Option<usize>,
    /// Indices of skeleton nodes, used as joints in this skin.
    pub joints: Vec<usize>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Skin {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.inverse_bind_matrices.as_ref() {
            serializer.property("inverseBindMatrices");
            serializer.value(&v);
        }
        if let Some(v) = self.skeleton.as_ref() {
            serializer.property("skeleton");
            serializer.value(&v);
        }
        serializer.property("joints");
        serializer.value(&self.joints);
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Skin {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut inverse_bind_matrices = None;
        let mut skeleton = None;
        let mut joints = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "inverseBindMatrices" => {
                    inverse_bind_matrices = Some(<usize>::deserialize(deserializer)?)
                }
                "skeleton" => skeleton = Some(<usize>::deserialize(deserializer)?),
                "joints" => joints = Some(<Vec<usize>>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            inverse_bind_matrices: inverse_bind_matrices,
            skeleton: skeleton,
            joints: joints?,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The root nodes of a scene.
#[derive(Debug, Clone)]
pub struct Scene {
    /// The indices of each root node.
    pub nodes: Vec<usize>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Scene {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if !self.nodes.is_empty() {
            serializer.property("nodes");
            serializer.value(&self.nodes);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Scene {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut nodes = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "nodes" => nodes = Some(<Vec<usize>>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            nodes: nodes.unwrap_or_else(Vec::new),
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Debug, Clone)]
pub struct Sampler {
    /// Magnification filter.
    pub mag_filter: Option<SamplerMagFilter>,
    /// Minification filter.
    pub min_filter: Option<SamplerMinFilter>,
    /// S (U) wrapping mode.
    pub wrap_s: SamplerWrapS,
    /// T (V) wrapping mode.
    pub wrap_t: SamplerWrapT,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Sampler {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.mag_filter.as_ref() {
            serializer.property("magFilter");
            serializer.value(&v);
        }
        if let Some(v) = self.min_filter.as_ref() {
            serializer.property("minFilter");
            serializer.value(&v);
        }
        if self.wrap_s != SamplerWrapS::Repeat {
            serializer.property("wrapS");
            serializer.value(&self.wrap_s);
        }
        if self.wrap_t != SamplerWrapT::Repeat {
            serializer.property("wrapT");
            serializer.value(&self.wrap_t);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Sampler {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut mag_filter = None;
        let mut min_filter = None;
        let mut wrap_s = None;
        let mut wrap_t = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "magFilter" => mag_filter = Some(<SamplerMagFilter>::deserialize(deserializer)?),
                "minFilter" => min_filter = Some(<SamplerMinFilter>::deserialize(deserializer)?),
                "wrapS" => wrap_s = Some(<SamplerWrapS>::deserialize(deserializer)?),
                "wrapT" => wrap_t = Some(<SamplerWrapT>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            mag_filter: mag_filter,
            min_filter: min_filter,
            wrap_s: wrap_s.map_or_else(|| SamplerWrapS::Repeat, |m| m),
            wrap_t: wrap_t.map_or_else(|| SamplerWrapT::Repeat, |m| m),
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// T (V) wrapping mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerWrapT {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
}

impl<S: Serializer> Serialize<S> for SamplerWrapT {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::ClampToEdge => 33071.serialize(serializer),
            Self::MirroredRepeat => 33648.serialize(serializer),
            Self::Repeat => 10497.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for SamplerWrapT {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            33071 => Self::ClampToEdge,
            33648 => Self::MirroredRepeat,
            10497 => Self::Repeat,
            _ => None?,
        })
    }
}

/// S (U) wrapping mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerWrapS {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
}

impl<S: Serializer> Serialize<S> for SamplerWrapS {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::ClampToEdge => 33071.serialize(serializer),
            Self::MirroredRepeat => 33648.serialize(serializer),
            Self::Repeat => 10497.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for SamplerWrapS {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            33071 => Self::ClampToEdge,
            33648 => Self::MirroredRepeat,
            10497 => Self::Repeat,
            _ => None?,
        })
    }
}

/// Minification filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerMinFilter {
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}

impl<S: Serializer> Serialize<S> for SamplerMinFilter {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Nearest => 9728.serialize(serializer),
            Self::Linear => 9729.serialize(serializer),
            Self::NearestMipmapNearest => 9984.serialize(serializer),
            Self::LinearMipmapNearest => 9985.serialize(serializer),
            Self::NearestMipmapLinear => 9986.serialize(serializer),
            Self::LinearMipmapLinear => 9987.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for SamplerMinFilter {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            9728 => Self::Nearest,
            9729 => Self::Linear,
            9984 => Self::NearestMipmapNearest,
            9985 => Self::LinearMipmapNearest,
            9986 => Self::NearestMipmapLinear,
            9987 => Self::LinearMipmapLinear,
            _ => None?,
        })
    }
}

/// Magnification filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerMagFilter {
    Nearest = 9728,
    Linear = 9729,
}

impl<S: Serializer> Serialize<S> for SamplerMagFilter {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Nearest => 9728.serialize(serializer),
            Self::Linear => 9729.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for SamplerMagFilter {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            9728 => Self::Nearest,
            9729 => Self::Linear,
            _ => None?,
        })
    }
}

/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present.
#[derive(Debug, Clone)]
pub struct Node {
    /// The index of the camera referenced by this node.
    pub camera: Option<usize>,
    /// The indices of this node's children.
    pub children: Vec<usize>,
    /// The index of the skin referenced by this node.
    pub skin: Option<usize>,
    /// A floating-point 4x4 transformation matrix stored in column-major order.
    pub matrix: Option<[f32; 16]>,
    /// The index of the mesh in this node.
    pub mesh: Option<usize>,
    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
    pub rotation: Option<[f32; 4]>,
    /// The node's non-uniform scale, given as the scaling factors along the x, y, and z axes.
    pub scale: Option<[f32; 3]>,
    /// The node's translation along the x, y, and z axes.
    pub translation: Option<[f32; 3]>,
    /// The weights of the instantiated morph target. The number of array elements **MUST** match the number of morph targets of the referenced mesh. When defined, `mesh` **MUST** also be defined.
    pub weights: Vec<f32>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Node {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.camera.as_ref() {
            serializer.property("camera");
            serializer.value(&v);
        }
        if !self.children.is_empty() {
            serializer.property("children");
            serializer.value(&self.children);
        }
        if let Some(v) = self.skin.as_ref() {
            serializer.property("skin");
            serializer.value(&v);
        }
        if let Some(v) = self.matrix.as_ref() {
            serializer.property("matrix");
            serializer.value(&v);
        }
        if let Some(v) = self.mesh.as_ref() {
            serializer.property("mesh");
            serializer.value(&v);
        }
        if let Some(v) = self.rotation.as_ref() {
            serializer.property("rotation");
            serializer.value(&v);
        }
        if let Some(v) = self.scale.as_ref() {
            serializer.property("scale");
            serializer.value(&v);
        }
        if let Some(v) = self.translation.as_ref() {
            serializer.property("translation");
            serializer.value(&v);
        }
        if !self.weights.is_empty() {
            serializer.property("weights");
            serializer.value(&self.weights);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Node {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut camera = None;
        let mut children = None;
        let mut skin = None;
        let mut matrix = None;
        let mut mesh = None;
        let mut rotation = None;
        let mut scale = None;
        let mut translation = None;
        let mut weights = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "camera" => camera = Some(<usize>::deserialize(deserializer)?),
                "children" => children = Some(<Vec<usize>>::deserialize(deserializer)?),
                "skin" => skin = Some(<usize>::deserialize(deserializer)?),
                "matrix" => matrix = Some(<[f32; 16]>::deserialize(deserializer)?),
                "mesh" => mesh = Some(<usize>::deserialize(deserializer)?),
                "rotation" => rotation = Some(<[f32; 4]>::deserialize(deserializer)?),
                "scale" => scale = Some(<[f32; 3]>::deserialize(deserializer)?),
                "translation" => translation = Some(<[f32; 3]>::deserialize(deserializer)?),
                "weights" => weights = Some(<Vec<f32>>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            camera: camera,
            children: children.unwrap_or_else(Vec::new),
            skin: skin,
            matrix: if translation.is_none() && rotation.is_none() && scale.is_none() {
                Some(matrix.map_or_else(
                    || {
                        [
                            1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32,
                            0f32, 0f32, 0f32, 1f32,
                        ]
                    },
                    |m| m,
                ))
            } else {
                None
            },
            mesh: mesh,
            rotation: if matrix.is_none() {
                Some(rotation.map_or_else(|| [0f32, 0f32, 0f32, 1f32], |m| m))
            } else {
                None
            },
            scale: if matrix.is_none() {
                Some(scale.map_or_else(|| [1f32, 1f32, 1f32], |m| m))
            } else {
                None
            },
            translation: if matrix.is_none() {
                Some(translation.map_or_else(|| [0f32, 0f32, 0f32], |m| m))
            } else {
                None
            },
            weights: weights.unwrap_or_else(Vec::new),
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// A set of primitives to be rendered.  Its global transform is defined by a node that references it.
#[derive(Debug, Clone)]
pub struct Mesh {
    /// An array of primitives, each defining geometry to be rendered.
    pub primitives: Vec<MeshPrimitive>,
    /// Array of weights to be applied to the morph targets. The number of array elements **MUST** match the number of morph targets.
    pub weights: Vec<f32>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Mesh {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("primitives");
        serializer.value(&self.primitives);
        if !self.weights.is_empty() {
            serializer.property("weights");
            serializer.value(&self.weights);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Mesh {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut primitives = None;
        let mut weights = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "primitives" => primitives = Some(<Vec<MeshPrimitive>>::deserialize(deserializer)?),
                "weights" => weights = Some(<Vec<f32>>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            primitives: primitives?,
            weights: weights.unwrap_or_else(Vec::new),
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Geometry to be rendered with the given material.
#[derive(Debug, Clone)]
pub struct MeshPrimitive {
    /// A plain JSON object, where each key corresponds to a mesh attribute semantic and each value is the index of the accessor containing attribute's data.
    pub attributes: HashMap<String, usize>,
    /// The index of the accessor that contains the vertex indices.
    pub indices: Option<usize>,
    /// The index of the material to apply to this primitive when rendering.
    pub material: Option<usize>,
    /// The topology type of primitives to render.
    pub mode: MeshPrimitiveMode,
    /// An array of morph targets.
    pub targets: Vec<HashMap<String, usize>>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for MeshPrimitive {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("attributes");
        serializer.value(&self.attributes);
        if let Some(v) = self.indices.as_ref() {
            serializer.property("indices");
            serializer.value(&v);
        }
        if let Some(v) = self.material.as_ref() {
            serializer.property("material");
            serializer.value(&v);
        }
        if self.mode != MeshPrimitiveMode::Triangles {
            serializer.property("mode");
            serializer.value(&self.mode);
        }
        if !self.targets.is_empty() {
            serializer.property("targets");
            serializer.value(&self.targets);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MeshPrimitive {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut attributes = None;
        let mut indices = None;
        let mut material = None;
        let mut mode = None;
        let mut targets = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "attributes" => {
                    attributes = Some(<HashMap<String, usize>>::deserialize(deserializer)?)
                }
                "indices" => indices = Some(<usize>::deserialize(deserializer)?),
                "material" => material = Some(<usize>::deserialize(deserializer)?),
                "mode" => mode = Some(<MeshPrimitiveMode>::deserialize(deserializer)?),
                "targets" => {
                    targets = Some(<Vec<HashMap<String, usize>>>::deserialize(deserializer)?)
                }
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            attributes: attributes?,
            indices: indices,
            material: material,
            mode: mode.map_or_else(|| MeshPrimitiveMode::Triangles, |m| m),
            targets: targets.unwrap_or_else(Vec::new),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The topology type of primitives to render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshPrimitiveMode {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

impl<S: Serializer> Serialize<S> for MeshPrimitiveMode {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Points => 0.serialize(serializer),
            Self::Lines => 1.serialize(serializer),
            Self::LineLoop => 2.serialize(serializer),
            Self::LineStrip => 3.serialize(serializer),
            Self::Triangles => 4.serialize(serializer),
            Self::TriangleStrip => 5.serialize(serializer),
            Self::TriangleFan => 6.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MeshPrimitiveMode {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            0 => Self::Points,
            1 => Self::Lines,
            2 => Self::LineLoop,
            3 => Self::LineStrip,
            4 => Self::Triangles,
            5 => Self::TriangleStrip,
            6 => Self::TriangleFan,
            _ => None?,
        })
    }
}

/// The material appearance of a primitive.
#[derive(Debug, Clone)]
pub struct Material {
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
    /// A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply.
    pub pbr_metallic_roughness: Option<MaterialPbrMetallicRoughness>,
    /// The tangent space normal texture.
    pub normal_texture: Option<MaterialNormalTextureInfo>,
    /// The occlusion texture.
    pub occlusion_texture: Option<MaterialOcclusionTextureInfo>,
    /// The emissive texture.
    pub emissive_texture: Option<TextureInfo>,
    /// The factors for the emissive color of the material.
    pub emissive_factor: [f32; 3],
    /// The alpha rendering mode of the material.
    pub alpha_mode: MaterialAlphaMode,
    /// The alpha cutoff value of the material.
    pub alpha_cutoff: f32,
    /// Specifies whether the material is double sided.
    pub double_sided: bool,
}

impl<S: Serializer> Serialize<S> for Material {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        if let Some(v) = self.pbr_metallic_roughness.as_ref() {
            serializer.property("pbrMetallicRoughness");
            serializer.value(&v);
        }
        if let Some(v) = self.normal_texture.as_ref() {
            serializer.property("normalTexture");
            serializer.value(&v);
        }
        if let Some(v) = self.occlusion_texture.as_ref() {
            serializer.property("occlusionTexture");
            serializer.value(&v);
        }
        if let Some(v) = self.emissive_texture.as_ref() {
            serializer.property("emissiveTexture");
            serializer.value(&v);
        }
        if self.emissive_factor != [0f32, 0f32, 0f32] {
            serializer.property("emissiveFactor");
            serializer.value(&self.emissive_factor);
        }
        if self.alpha_mode != MaterialAlphaMode::Opaque {
            serializer.property("alphaMode");
            serializer.value(&self.alpha_mode);
        }
        if self.alpha_cutoff != 0.5f32 {
            serializer.property("alphaCutoff");
            serializer.value(&self.alpha_cutoff);
        }
        if self.double_sided {
            serializer.property("doubleSided");
            serializer.value(&self.double_sided);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Material {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;
        let mut pbr_metallic_roughness = None;
        let mut normal_texture = None;
        let mut occlusion_texture = None;
        let mut emissive_texture = None;
        let mut emissive_factor = None;
        let mut alpha_mode = None;
        let mut alpha_cutoff = None;
        let mut double_sided = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                "pbrMetallicRoughness" => {
                    pbr_metallic_roughness =
                        Some(<MaterialPbrMetallicRoughness>::deserialize(deserializer)?)
                }
                "normalTexture" => {
                    normal_texture = Some(<MaterialNormalTextureInfo>::deserialize(deserializer)?)
                }
                "occlusionTexture" => {
                    occlusion_texture =
                        Some(<MaterialOcclusionTextureInfo>::deserialize(deserializer)?)
                }
                "emissiveTexture" => {
                    emissive_texture = Some(<TextureInfo>::deserialize(deserializer)?)
                }
                "emissiveFactor" => emissive_factor = Some(<[f32; 3]>::deserialize(deserializer)?),
                "alphaMode" => alpha_mode = Some(<MaterialAlphaMode>::deserialize(deserializer)?),
                "alphaCutoff" => alpha_cutoff = Some(<f32>::deserialize(deserializer)?),
                "doubleSided" => double_sided = Some(<bool>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
            pbr_metallic_roughness: pbr_metallic_roughness,
            normal_texture: normal_texture,
            occlusion_texture: occlusion_texture,
            emissive_texture: emissive_texture,
            emissive_factor: emissive_factor.map_or_else(|| [0f32, 0f32, 0f32], |m| m),
            alpha_mode: alpha_mode.map_or_else(|| MaterialAlphaMode::Opaque, |m| m),
            alpha_cutoff: alpha_cutoff.map_or_else(|| 0.5f32, |m| m),
            double_sided: double_sided.map_or_else(|| false, |m| m),
        })
    }
}

/// The alpha rendering mode of the material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialAlphaMode {
    /// The alpha value is ignored, and the rendered output is fully opaque.
    Opaque,
    /// The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified `alphaCutoff` value; the exact appearance of the edges **MAY** be subject to implementation-specific techniques such as "`Alpha-to-Coverage`".
    Mask,
    /// The alpha value is used to composite the source and destination areas. The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
    Blend,
}

impl<S: Serializer> Serialize<S> for MaterialAlphaMode {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Opaque => "OPAQUE".serialize(serializer),
            Self::Mask => "MASK".serialize(serializer),
            Self::Blend => "BLEND".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MaterialAlphaMode {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "OPAQUE" => Self::Opaque,
            "MASK" => Self::Mask,
            "BLEND" => Self::Blend,
            _ => None?,
        })
    }
}

/// The occlusion texture.
#[derive(Debug, Clone)]
pub struct MaterialOcclusionTextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
    pub tex_coord: usize,
    /// A scalar multiplier controlling the amount of occlusion applied.
    pub strength: f32,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for MaterialOcclusionTextureInfo {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("index");
        serializer.value(&self.index);
        if self.tex_coord != 0usize {
            serializer.property("texCoord");
            serializer.value(&self.tex_coord);
        }
        if self.strength != 1f32 {
            serializer.property("strength");
            serializer.value(&self.strength);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MaterialOcclusionTextureInfo {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut index = None;
        let mut tex_coord = None;
        let mut strength = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "index" => index = Some(<usize>::deserialize(deserializer)?),
                "texCoord" => tex_coord = Some(<usize>::deserialize(deserializer)?),
                "strength" => strength = Some(<f32>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            index: index?,
            tex_coord: tex_coord.map_or_else(|| 0usize, |m| m),
            strength: strength.map_or_else(|| 1f32, |m| m),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The tangent space normal texture.
#[derive(Debug, Clone)]
pub struct MaterialNormalTextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
    pub tex_coord: usize,
    /// The scalar parameter applied to each normal vector of the normal texture.
    pub scale: f32,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for MaterialNormalTextureInfo {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("index");
        serializer.value(&self.index);
        if self.tex_coord != 0usize {
            serializer.property("texCoord");
            serializer.value(&self.tex_coord);
        }
        if self.scale != 1f32 {
            serializer.property("scale");
            serializer.value(&self.scale);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MaterialNormalTextureInfo {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut index = None;
        let mut tex_coord = None;
        let mut scale = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "index" => index = Some(<usize>::deserialize(deserializer)?),
                "texCoord" => tex_coord = Some(<usize>::deserialize(deserializer)?),
                "scale" => scale = Some(<f32>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            index: index?,
            tex_coord: tex_coord.map_or_else(|| 0usize, |m| m),
            scale: scale.map_or_else(|| 1f32, |m| m),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply.
#[derive(Debug, Clone)]
pub struct MaterialPbrMetallicRoughness {
    /// The factors for the base color of the material.
    pub base_color_factor: [f32; 4],
    /// The base color texture.
    pub base_color_texture: Option<TextureInfo>,
    /// The factor for the metalness of the material.
    pub metallic_factor: f32,
    /// The factor for the roughness of the material.
    pub roughness_factor: f32,
    /// The metallic-roughness texture.
    pub metallic_roughness_texture: Option<TextureInfo>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for MaterialPbrMetallicRoughness {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if self.base_color_factor != [1f32, 1f32, 1f32, 1f32] {
            serializer.property("baseColorFactor");
            serializer.value(&self.base_color_factor);
        }
        if let Some(v) = self.base_color_texture.as_ref() {
            serializer.property("baseColorTexture");
            serializer.value(&v);
        }
        if self.metallic_factor != 1f32 {
            serializer.property("metallicFactor");
            serializer.value(&self.metallic_factor);
        }
        if self.roughness_factor != 1f32 {
            serializer.property("roughnessFactor");
            serializer.value(&self.roughness_factor);
        }
        if let Some(v) = self.metallic_roughness_texture.as_ref() {
            serializer.property("metallicRoughnessTexture");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for MaterialPbrMetallicRoughness {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut base_color_factor = None;
        let mut base_color_texture = None;
        let mut metallic_factor = None;
        let mut roughness_factor = None;
        let mut metallic_roughness_texture = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "baseColorFactor" => {
                    base_color_factor = Some(<[f32; 4]>::deserialize(deserializer)?)
                }
                "baseColorTexture" => {
                    base_color_texture = Some(<TextureInfo>::deserialize(deserializer)?)
                }
                "metallicFactor" => metallic_factor = Some(<f32>::deserialize(deserializer)?),
                "roughnessFactor" => roughness_factor = Some(<f32>::deserialize(deserializer)?),
                "metallicRoughnessTexture" => {
                    metallic_roughness_texture = Some(<TextureInfo>::deserialize(deserializer)?)
                }
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            base_color_factor: base_color_factor.map_or_else(|| [1f32, 1f32, 1f32, 1f32], |m| m),
            base_color_texture: base_color_texture,
            metallic_factor: metallic_factor.map_or_else(|| 1f32, |m| m),
            roughness_factor: roughness_factor.map_or_else(|| 1f32, |m| m),
            metallic_roughness_texture: metallic_roughness_texture,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The base color texture.
#[derive(Debug, Clone)]
pub struct TextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture's TEXCOORD attribute used for texture coordinate mapping.
    pub tex_coord: usize,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for TextureInfo {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("index");
        serializer.value(&self.index);
        if self.tex_coord != 0usize {
            serializer.property("texCoord");
            serializer.value(&self.tex_coord);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for TextureInfo {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut index = None;
        let mut tex_coord = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "index" => index = Some(<usize>::deserialize(deserializer)?),
                "texCoord" => tex_coord = Some(<usize>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            index: index?,
            tex_coord: tex_coord.map_or_else(|| 0usize, |m| m),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Image data used to create a texture. Image **MAY** be referenced by an URI (or IRI) or a buffer view index.
#[derive(Debug, Clone)]
pub struct Image {
    /// The URI (or IRI) of the image.
    pub uri: Option<String>,
    /// The image's media type. This field **MUST** be defined when `bufferView` is defined.
    pub mime_type: Option<ImageMimeType>,
    /// The index of the bufferView that contains the image. This field **MUST NOT** be defined when `uri` is defined.
    pub buffer_view: Option<usize>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Image {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.uri.as_ref() {
            serializer.property("uri");
            serializer.value(&v);
        }
        if let Some(v) = self.mime_type.as_ref() {
            serializer.property("mimeType");
            serializer.value(&v);
        }
        if let Some(v) = self.buffer_view.as_ref() {
            serializer.property("bufferView");
            serializer.value(&v);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Image {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut uri = None;
        let mut mime_type = None;
        let mut buffer_view = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "uri" => uri = Some(<String>::deserialize(deserializer)?),
                "mimeType" => mime_type = Some(<ImageMimeType>::deserialize(deserializer)?),
                "bufferView" => buffer_view = Some(<usize>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            uri: uri,
            mime_type: mime_type,
            buffer_view: buffer_view,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The image's media type. This field **MUST** be defined when `bufferView` is defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageMimeType {
    ImageJpeg,
    ImagePng,
}

impl<S: Serializer> Serialize<S> for ImageMimeType {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::ImageJpeg => "image/jpeg".serialize(serializer),
            Self::ImagePng => "image/png".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for ImageMimeType {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "image/jpeg" => Self::ImageJpeg,
            "image/png" => Self::ImagePng,
            _ => None?,
        })
    }
}

/// A camera's projection.  A node **MAY** reference a camera to apply a transform to place the camera in the scene.
#[derive(Debug, Clone)]
pub struct Camera {
    /// An orthographic camera containing properties to create an orthographic projection matrix. This property **MUST NOT** be defined when `perspective` is defined.
    pub orthographic: Option<CameraOrthographic>,
    /// A perspective camera containing properties to create a perspective projection matrix. This property **MUST NOT** be defined when `orthographic` is defined.
    pub perspective: Option<CameraPerspective>,
    /// Specifies if the camera uses a perspective or orthographic projection.
    pub type_: CameraType,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Camera {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.orthographic.as_ref() {
            serializer.property("orthographic");
            serializer.value(&v);
        }
        if let Some(v) = self.perspective.as_ref() {
            serializer.property("perspective");
            serializer.value(&v);
        }
        serializer.property("type");
        serializer.value(&self.type_);
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Camera {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut orthographic = None;
        let mut perspective = None;
        let mut type_ = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "orthographic" => {
                    orthographic = Some(<CameraOrthographic>::deserialize(deserializer)?)
                }
                "perspective" => {
                    perspective = Some(<CameraPerspective>::deserialize(deserializer)?)
                }
                "type" => type_ = Some(<CameraType>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            orthographic: if perspective.is_none() {
                orthographic.clone()
            } else {
                None
            },
            perspective: if orthographic.is_none() {
                perspective.clone()
            } else {
                None
            },
            type_: type_?,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Specifies if the camera uses a perspective or orthographic projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

impl<S: Serializer> Serialize<S> for CameraType {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Perspective => "perspective".serialize(serializer),
            Self::Orthographic => "orthographic".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for CameraType {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "perspective" => Self::Perspective,
            "orthographic" => Self::Orthographic,
            _ => None?,
        })
    }
}

/// A perspective camera containing properties to create a perspective projection matrix. This property **MUST NOT** be defined when `orthographic` is defined.
#[derive(Debug, Clone)]
pub struct CameraPerspective {
    /// The floating-point aspect ratio of the field of view.
    pub aspect_ratio: Option<f32>,
    /// The floating-point vertical field of view in radians. This value **SHOULD** be less than π.
    pub yfov: f32,
    /// The floating-point distance to the far clipping plane.
    pub zfar: Option<f32>,
    /// The floating-point distance to the near clipping plane.
    pub znear: f32,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for CameraPerspective {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.aspect_ratio.as_ref() {
            serializer.property("aspectRatio");
            serializer.value(&v);
        }
        serializer.property("yfov");
        serializer.value(&self.yfov);
        if let Some(v) = self.zfar.as_ref() {
            serializer.property("zfar");
            serializer.value(&v);
        }
        serializer.property("znear");
        serializer.value(&self.znear);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for CameraPerspective {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut aspect_ratio = None;
        let mut yfov = None;
        let mut zfar = None;
        let mut znear = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "aspectRatio" => aspect_ratio = Some(<f32>::deserialize(deserializer)?),
                "yfov" => yfov = Some(<f32>::deserialize(deserializer)?),
                "zfar" => zfar = Some(<f32>::deserialize(deserializer)?),
                "znear" => znear = Some(<f32>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            aspect_ratio: aspect_ratio,
            yfov: yfov?,
            zfar: zfar,
            znear: znear?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// An orthographic camera containing properties to create an orthographic projection matrix. This property **MUST NOT** be defined when `perspective` is defined.
#[derive(Debug, Clone)]
pub struct CameraOrthographic {
    /// The floating-point horizontal magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
    pub xmag: f32,
    /// The floating-point vertical magnification of the view. This value **MUST NOT** be equal to zero. This value **SHOULD NOT** be negative.
    pub ymag: f32,
    /// The floating-point distance to the far clipping plane. This value **MUST NOT** be equal to zero. `zfar` **MUST** be greater than `znear`.
    pub zfar: f32,
    /// The floating-point distance to the near clipping plane.
    pub znear: f32,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for CameraOrthographic {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("xmag");
        serializer.value(&self.xmag);
        serializer.property("ymag");
        serializer.value(&self.ymag);
        serializer.property("zfar");
        serializer.value(&self.zfar);
        serializer.property("znear");
        serializer.value(&self.znear);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for CameraOrthographic {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut xmag = None;
        let mut ymag = None;
        let mut zfar = None;
        let mut znear = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "xmag" => xmag = Some(<f32>::deserialize(deserializer)?),
                "ymag" => ymag = Some(<f32>::deserialize(deserializer)?),
                "zfar" => zfar = Some(<f32>::deserialize(deserializer)?),
                "znear" => znear = Some(<f32>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            xmag: xmag?,
            ymag: ymag?,
            zfar: zfar?,
            znear: znear?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Debug, Clone)]
pub struct BufferView {
    /// The index of the buffer.
    pub buffer: usize,
    /// The offset into the buffer in bytes.
    pub byte_offset: usize,
    /// The length of the bufferView in bytes.
    pub byte_length: usize,
    /// The stride, in bytes.
    pub byte_stride: Option<usize>,
    /// The hint representing the intended GPU buffer type to use with this buffer view.
    pub target: Option<BufferViewTarget>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for BufferView {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("buffer");
        serializer.value(&self.buffer);
        if self.byte_offset != 0usize {
            serializer.property("byteOffset");
            serializer.value(&self.byte_offset);
        }
        serializer.property("byteLength");
        serializer.value(&self.byte_length);
        if let Some(v) = self.byte_stride.as_ref() {
            serializer.property("byteStride");
            serializer.value(&v);
        }
        if let Some(v) = self.target.as_ref() {
            serializer.property("target");
            serializer.value(&v);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for BufferView {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut buffer = None;
        let mut byte_offset = None;
        let mut byte_length = None;
        let mut byte_stride = None;
        let mut target = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "buffer" => buffer = Some(<usize>::deserialize(deserializer)?),
                "byteOffset" => byte_offset = Some(<usize>::deserialize(deserializer)?),
                "byteLength" => byte_length = Some(<usize>::deserialize(deserializer)?),
                "byteStride" => byte_stride = Some(<usize>::deserialize(deserializer)?),
                "target" => target = Some(<BufferViewTarget>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            buffer: buffer?,
            byte_offset: byte_offset.map_or_else(|| 0usize, |m| m),
            byte_length: byte_length?,
            byte_stride: byte_stride,
            target: target,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The hint representing the intended GPU buffer type to use with this buffer view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

impl<S: Serializer> Serialize<S> for BufferViewTarget {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::ArrayBuffer => 34962.serialize(serializer),
            Self::ElementArrayBuffer => 34963.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for BufferViewTarget {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            34962 => Self::ArrayBuffer,
            34963 => Self::ElementArrayBuffer,
            _ => None?,
        })
    }
}

/// A buffer points to binary geometry, animation, or skins.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The URI (or IRI) of the buffer.
    pub uri: Option<String>,
    /// The length of the buffer in bytes.
    pub byte_length: usize,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Buffer {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.uri.as_ref() {
            serializer.property("uri");
            serializer.value(&v);
        }
        serializer.property("byteLength");
        serializer.value(&self.byte_length);
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Buffer {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut uri = None;
        let mut byte_length = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "uri" => uri = Some(<String>::deserialize(deserializer)?),
                "byteLength" => byte_length = Some(<usize>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            uri: uri,
            byte_length: byte_length?,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Metadata about the glTF asset.
#[derive(Debug, Clone)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<String>,
    /// Tool that generated this glTF model.  Useful for debugging.
    pub generator: Option<String>,
    /// The glTF version in the form of `<major>.<minor>` that this asset targets.
    pub version: String,
    /// The minimum glTF version in the form of `<major>.<minor>` that this asset targets. This property **MUST NOT** be greater than the asset version.
    pub min_version: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Asset {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.copyright.as_ref() {
            serializer.property("copyright");
            serializer.value(&v);
        }
        if let Some(v) = self.generator.as_ref() {
            serializer.property("generator");
            serializer.value(&v);
        }
        serializer.property("version");
        serializer.value(&self.version);
        if let Some(v) = self.min_version.as_ref() {
            serializer.property("minVersion");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Asset {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut copyright = None;
        let mut generator = None;
        let mut version = None;
        let mut min_version = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "copyright" => copyright = Some(<String>::deserialize(deserializer)?),
                "generator" => generator = Some(<String>::deserialize(deserializer)?),
                "version" => version = Some(<String>::deserialize(deserializer)?),
                "minVersion" => min_version = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            copyright: copyright,
            generator: generator,
            version: version?,
            min_version: min_version,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// A keyframe animation.
#[derive(Debug, Clone)]
pub struct Animation {
    /// An array of animation channels. An animation channel combines an animation sampler with a target property being animated. Different channels of the same animation **MUST NOT** have the same targets.
    pub channels: Vec<AnimationChannel>,
    /// An array of animation samplers. An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
    pub samplers: Vec<AnimationSampler>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Animation {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("channels");
        serializer.value(&self.channels);
        serializer.property("samplers");
        serializer.value(&self.samplers);
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Animation {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut channels = None;
        let mut samplers = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "channels" => channels = Some(<Vec<AnimationChannel>>::deserialize(deserializer)?),
                "samplers" => samplers = Some(<Vec<AnimationSampler>>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            channels: channels?,
            samplers: samplers?,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
#[derive(Debug, Clone)]
pub struct AnimationSampler {
    /// The index of an accessor containing keyframe timestamps.
    pub input: usize,
    /// Interpolation algorithm.
    pub interpolation: AnimationSamplerInterpolation,
    /// The index of an accessor, containing keyframe output values.
    pub output: usize,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AnimationSampler {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("input");
        serializer.value(&self.input);
        if self.interpolation != AnimationSamplerInterpolation::Linear {
            serializer.property("interpolation");
            serializer.value(&self.interpolation);
        }
        serializer.property("output");
        serializer.value(&self.output);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AnimationSampler {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut input = None;
        let mut interpolation = None;
        let mut output = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "input" => input = Some(<usize>::deserialize(deserializer)?),
                "interpolation" => {
                    interpolation =
                        Some(<AnimationSamplerInterpolation>::deserialize(deserializer)?)
                }
                "output" => output = Some(<usize>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            input: input?,
            interpolation: interpolation
                .map_or_else(|| AnimationSamplerInterpolation::Linear, |m| m),
            output: output?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Interpolation algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationSamplerInterpolation {
    /// The animated values are linearly interpolated between keyframes. When targeting a rotation, spherical linear interpolation (slerp) **SHOULD** be used to interpolate quaternions. The number of output elements **MUST** equal the number of input elements.
    Linear,
    /// The animated values remain constant to the output of the first keyframe, until the next keyframe. The number of output elements **MUST** equal the number of input elements.
    Step,
    /// The animation's interpolation is computed using a cubic spline with specified tangents. The number of output elements **MUST** equal three times the number of input elements. For each input element, the output stores three elements, an in-tangent, a spline vertex, and an out-tangent. There **MUST** be at least two keyframes when using this interpolation.
    Cubicspline,
}

impl<S: Serializer> Serialize<S> for AnimationSamplerInterpolation {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Linear => "LINEAR".serialize(serializer),
            Self::Step => "STEP".serialize(serializer),
            Self::Cubicspline => "CUBICSPLINE".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AnimationSamplerInterpolation {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "LINEAR" => Self::Linear,
            "STEP" => Self::Step,
            "CUBICSPLINE" => Self::Cubicspline,
            _ => None?,
        })
    }
}

/// An animation channel combines an animation sampler with a target property being animated.
#[derive(Debug, Clone)]
pub struct AnimationChannel {
    /// The index of a sampler in this animation used to compute the value for the target.
    pub sampler: usize,
    /// The descriptor of the animated property.
    pub target: AnimationChannelTarget,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AnimationChannel {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("sampler");
        serializer.value(&self.sampler);
        serializer.property("target");
        serializer.value(&self.target);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AnimationChannel {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut sampler = None;
        let mut target = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "sampler" => sampler = Some(<usize>::deserialize(deserializer)?),
                "target" => target = Some(<AnimationChannelTarget>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            sampler: sampler?,
            target: target?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The descriptor of the animated property.
#[derive(Debug, Clone)]
pub struct AnimationChannelTarget {
    /// The index of the node to animate. When undefined, the animated object **MAY** be defined by an extension.
    pub node: Option<usize>,
    /// The name of the node's TRS property to animate, or the `"weights"` of the Morph Targets it instantiates. For the `"translation"` property, the values that are provided by the sampler are the translation along the X, Y, and Z axes. For the `"rotation"` property, the values are a quaternion in the order (x, y, z, w), where w is the scalar. For the `"scale"` property, the values are the scaling factors along the X, Y, and Z axes.
    pub path: AnimationChannelTargetPath,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AnimationChannelTarget {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.node.as_ref() {
            serializer.property("node");
            serializer.value(&v);
        }
        serializer.property("path");
        serializer.value(&self.path);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AnimationChannelTarget {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut node = None;
        let mut path = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "node" => node = Some(<usize>::deserialize(deserializer)?),
                "path" => path = Some(<AnimationChannelTargetPath>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            node: node,
            path: path?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The name of the node's TRS property to animate, or the `"weights"` of the Morph Targets it instantiates. For the `"translation"` property, the values that are provided by the sampler are the translation along the X, Y, and Z axes. For the `"rotation"` property, the values are a quaternion in the order (x, y, z, w), where w is the scalar. For the `"scale"` property, the values are the scaling factors along the X, Y, and Z axes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationChannelTargetPath {
    Translation,
    Rotation,
    Scale,
    Weights,
}

impl<S: Serializer> Serialize<S> for AnimationChannelTargetPath {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Translation => "translation".serialize(serializer),
            Self::Rotation => "rotation".serialize(serializer),
            Self::Scale => "scale".serialize(serializer),
            Self::Weights => "weights".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AnimationChannelTargetPath {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "translation" => Self::Translation,
            "rotation" => Self::Rotation,
            "scale" => Self::Scale,
            "weights" => Self::Weights,
            _ => None?,
        })
    }
}

/// A typed view into a buffer view that contains raw binary data.
#[derive(Debug, Clone)]
pub struct Accessor {
    /// The index of the bufferView.
    pub buffer_view: Option<usize>,
    /// The offset relative to the start of the buffer view in bytes.
    pub byte_offset: usize,
    /// The datatype of the accessor's components.
    pub component_type: AccessorComponentType,
    /// Specifies whether integer data values are normalized before usage.
    pub normalized: bool,
    /// The number of elements referenced by this accessor.
    pub count: usize,
    /// Specifies if the accessor's elements are scalars, vectors, or matrices.
    pub type_: AccessorType,
    /// Maximum value of each component in this accessor.
    pub max: Vec<f32>,
    /// Minimum value of each component in this accessor.
    pub min: Vec<f32>,
    /// Sparse storage of elements that deviate from their initialization value.
    pub sparse: Option<AccessorSparse>,
    /// The user-defined name of this object.
    pub name: Option<String>,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for Accessor {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        if let Some(v) = self.buffer_view.as_ref() {
            serializer.property("bufferView");
            serializer.value(&v);
        }
        if self.byte_offset != 0usize {
            serializer.property("byteOffset");
            serializer.value(&self.byte_offset);
        }
        serializer.property("componentType");
        serializer.value(&self.component_type);
        if self.normalized {
            serializer.property("normalized");
            serializer.value(&self.normalized);
        }
        serializer.property("count");
        serializer.value(&self.count);
        serializer.property("type");
        serializer.value(&self.type_);
        if !self.max.is_empty() {
            serializer.property("max");
            serializer.value(&self.max);
        }
        if !self.min.is_empty() {
            serializer.property("min");
            serializer.value(&self.min);
        }
        if let Some(v) = self.sparse.as_ref() {
            serializer.property("sparse");
            serializer.value(&v);
        }
        if let Some(v) = self.name.as_ref() {
            serializer.property("name");
            serializer.value(&v);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Accessor {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut buffer_view = None;
        let mut byte_offset = None;
        let mut component_type = None;
        let mut normalized = None;
        let mut count = None;
        let mut type_ = None;
        let mut max = None;
        let mut min = None;
        let mut sparse = None;
        let mut name = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "bufferView" => buffer_view = Some(<usize>::deserialize(deserializer)?),
                "byteOffset" => byte_offset = Some(<usize>::deserialize(deserializer)?),
                "componentType" => {
                    component_type = Some(<AccessorComponentType>::deserialize(deserializer)?)
                }
                "normalized" => normalized = Some(<bool>::deserialize(deserializer)?),
                "count" => count = Some(<usize>::deserialize(deserializer)?),
                "type" => type_ = Some(<AccessorType>::deserialize(deserializer)?),
                "max" => max = Some(<Vec<f32>>::deserialize(deserializer)?),
                "min" => min = Some(<Vec<f32>>::deserialize(deserializer)?),
                "sparse" => sparse = Some(<AccessorSparse>::deserialize(deserializer)?),
                "name" => name = Some(<String>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            buffer_view: buffer_view,
            byte_offset: byte_offset.map_or_else(|| 0usize, |m| m),
            component_type: component_type?,
            normalized: normalized.map_or_else(|| false, |m| m),
            count: count?,
            type_: type_?,
            max: max.unwrap_or_else(Vec::new),
            min: min.unwrap_or_else(Vec::new),
            sparse: sparse,
            name: name,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// Sparse storage of elements that deviate from their initialization value.
#[derive(Debug, Clone)]
pub struct AccessorSparse {
    /// Number of deviating accessor values stored in the sparse array.
    pub count: usize,
    /// An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase.
    pub indices: AccessorSparseIndices,
    /// An object pointing to a buffer view containing the deviating accessor values.
    pub values: AccessorSparseValues,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AccessorSparse {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("count");
        serializer.value(&self.count);
        serializer.property("indices");
        serializer.value(&self.indices);
        serializer.property("values");
        serializer.value(&self.values);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorSparse {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut count = None;
        let mut indices = None;
        let mut values = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "count" => count = Some(<usize>::deserialize(deserializer)?),
                "indices" => indices = Some(<AccessorSparseIndices>::deserialize(deserializer)?),
                "values" => values = Some(<AccessorSparseValues>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            count: count?,
            indices: indices?,
            values: values?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// An object pointing to a buffer view containing the deviating accessor values.
#[derive(Debug, Clone)]
pub struct AccessorSparseValues {
    /// The index of the bufferView with sparse values. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined.
    pub buffer_view: usize,
    /// The offset relative to the start of the bufferView in bytes.
    pub byte_offset: usize,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AccessorSparseValues {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("bufferView");
        serializer.value(&self.buffer_view);
        if self.byte_offset != 0usize {
            serializer.property("byteOffset");
            serializer.value(&self.byte_offset);
        }
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorSparseValues {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut buffer_view = None;
        let mut byte_offset = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "bufferView" => buffer_view = Some(<usize>::deserialize(deserializer)?),
                "byteOffset" => byte_offset = Some(<usize>::deserialize(deserializer)?),
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            buffer_view: buffer_view?,
            byte_offset: byte_offset.map_or_else(|| 0usize, |m| m),
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase.
#[derive(Debug, Clone)]
pub struct AccessorSparseIndices {
    /// The index of the buffer view with sparse indices. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined. The buffer view and the optional `byteOffset` **MUST** be aligned to the `componentType` byte length.
    pub buffer_view: usize,
    /// The offset relative to the start of the buffer view in bytes.
    pub byte_offset: usize,
    /// The indices data type.
    pub component_type: AccessorSparseIndicesComponentType,
    /// JSON object with extension-specific objects.
    pub extensions: HashMap<String, ThingOwned>,
    /// Application-specific data.
    pub extras: Option<ThingOwned>,
}

impl<S: Serializer> Serialize<S> for AccessorSparseIndices {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("bufferView");
        serializer.value(&self.buffer_view);
        if self.byte_offset != 0usize {
            serializer.property("byteOffset");
            serializer.value(&self.byte_offset);
        }
        serializer.property("componentType");
        serializer.value(&self.component_type);
        if !self.extensions.is_empty() {
            serializer.property("extensions");
            serializer.value(&self.extensions);
        }
        if let Some(v) = self.extras.as_ref() {
            serializer.property("extras");
            serializer.value(&v);
        }
        serializer.end_object();
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorSparseIndices {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;
        let mut buffer_view = None;
        let mut byte_offset = None;
        let mut component_type = None;
        let mut extensions = None;
        let mut extras = None;

        while let Some(property) = deserializer.has_property() {
            match &*property {
                "bufferView" => buffer_view = Some(<usize>::deserialize(deserializer)?),
                "byteOffset" => byte_offset = Some(<usize>::deserialize(deserializer)?),
                "componentType" => {
                    component_type = Some(<AccessorSparseIndicesComponentType>::deserialize(
                        deserializer,
                    )?)
                }
                "extensions" => {
                    extensions = Some(<HashMap<String, ThingOwned>>::deserialize(deserializer)?)
                }
                "extras" => extras = Some(<ThingOwned>::deserialize(deserializer)?),
                _ => {}
            }
        }

        deserializer.end_object();
        Some(Self {
            buffer_view: buffer_view?,
            byte_offset: byte_offset.map_or_else(|| 0usize, |m| m),
            component_type: component_type?,
            extensions: extensions.unwrap_or_else(HashMap::new),
            extras: extras,
        })
    }
}

/// The indices data type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessorSparseIndicesComponentType {
    UnsignedByte = 5121,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
}

impl<S: Serializer> Serialize<S> for AccessorSparseIndicesComponentType {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::UnsignedByte => 5121.serialize(serializer),
            Self::UnsignedShort => 5123.serialize(serializer),
            Self::UnsignedInt => 5125.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorSparseIndicesComponentType {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            5121 => Self::UnsignedByte,
            5123 => Self::UnsignedShort,
            5125 => Self::UnsignedInt,
            _ => None?,
        })
    }
}

/// Specifies if the accessor's elements are scalars, vectors, or matrices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl<S: Serializer> Serialize<S> for AccessorType {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Scalar => "SCALAR".serialize(serializer),
            Self::Vec2 => "VEC2".serialize(serializer),
            Self::Vec3 => "VEC3".serialize(serializer),
            Self::Vec4 => "VEC4".serialize(serializer),
            Self::Mat2 => "MAT2".serialize(serializer),
            Self::Mat3 => "MAT3".serialize(serializer),
            Self::Mat4 => "MAT4".serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorType {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.string()?;
        Some(match &*value {
            "SCALAR" => Self::Scalar,
            "VEC2" => Self::Vec2,
            "VEC3" => Self::Vec3,
            "VEC4" => Self::Vec4,
            "MAT2" => Self::Mat2,
            "MAT3" => Self::Mat3,
            "MAT4" => Self::Mat4,
            _ => None?,
        })
    }
}

/// The datatype of the accessor's components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessorComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

impl<S: Serializer> Serialize<S> for AccessorComponentType {
    fn serialize(&self, serializer: &mut S) {
        match self {
            Self::Byte => 5120.serialize(serializer),
            Self::UnsignedByte => 5121.serialize(serializer),
            Self::Short => 5122.serialize(serializer),
            Self::UnsignedShort => 5123.serialize(serializer),
            Self::UnsignedInt => 5125.serialize(serializer),
            Self::Float => 5126.serialize(serializer),
        }
    }
}
impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for AccessorComponentType {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        let value = deserializer.i64()?;
        Some(match value {
            5120 => Self::Byte,
            5121 => Self::UnsignedByte,
            5122 => Self::Short,
            5123 => Self::UnsignedShort,
            5125 => Self::UnsignedInt,
            5126 => Self::Float,
            _ => None?,
        })
    }
}
