use crate::*;
use kgraphics::*;

pub struct Mesh {
    pub gpu_mesh: Option<GPUMesh>,
    pub mesh_data: Option<MeshData>,
    pub bounding_box: Option<BoundingBox<f32, 3>>,
}

impl Mesh {
    pub fn new(graphics: &mut Graphics, mesh_data: MeshData) -> Self {
        let gpu_mesh = graphics.new_gpu_mesh(&mesh_data).unwrap();
        let bounding_box = BoundingBox::<f32, 3>::from_points(&mesh_data.positions);
        Mesh {
            gpu_mesh: Some(gpu_mesh),
            mesh_data: Some(mesh_data),
            bounding_box: Some(bounding_box),
        }
    }
}

#[derive(Clone, Debug, Component, Default)]
pub struct MeshData {
    pub positions: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    pub normals: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    pub colors: Vec<Vec4>,
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
