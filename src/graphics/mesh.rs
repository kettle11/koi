use crate::*;
use kgraphics::*;

pub struct Mesh {
    pub gpu_mesh: Option<GPUMesh>,
    pub mesh_data: Option<MeshData>,
    pub bounding_box: Option<Box3>,
}

impl Mesh {
    pub fn new(graphics: &mut Graphics, mesh_data: MeshData) -> Self {
        let gpu_mesh = graphics.new_gpu_mesh(&mesh_data).unwrap();
        let bounding_box = Box3::from_points(&mesh_data.positions);
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

impl MeshData {
    pub fn new() -> Self {
        MeshData {
            positions: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            texture_coordinates: Vec::new(),
            colors: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
        self.indices.clear();
        self.normals.clear();
        self.texture_coordinates.clear();
        self.colors.clear();
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

impl MeshAssetLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl AssetLoader<Mesh> for MeshAssetLoader {
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
