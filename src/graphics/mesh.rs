use crate::*;
use kgraphics::*;

pub struct Mesh {
    pub gpu_mesh: Option<GPUMesh>,
    pub mesh_data: Option<MeshData>,
}

#[derive(Clone, Debug, Component)]
pub struct MeshData {
    pub positions: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    pub normals: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    pub colors: Vec<Vec4>,
}

impl Default for MeshData {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            texture_coordinates: Vec::new(),
            colors: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct GPUMesh {
    pub positions: DataBuffer<Vec3>,
    pub texture_coordinates: Option<DataBuffer<Vec2>>,
    pub normals: Option<DataBuffer<Vec3>>,
    pub index_buffer: IndexBuffer,
    pub triangle_count: u32,
    pub colors: Option<DataBuffer<Vec4>>,
}

pub struct MeshAssetLoader {}
impl AssetLoader<Mesh> for MeshAssetLoader {
    fn new() -> Self {
        Self {}
    }

    fn load_with_options(
        &mut self,
        _path: &str,
        _handle: Handle<Mesh>,
        _options: <Mesh as LoadableAssetTrait>::Options,
    ) {
        todo!()
    }
}

impl LoadableAssetTrait for Mesh {
    type Options = ();
    type AssetLoader = MeshAssetLoader;
}
