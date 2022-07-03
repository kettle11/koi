use crate::*;
use kgraphics::*;

pub struct Mesh {
    pub(crate) gpu_mesh: Option<GPUMesh>,
    pub mesh_data: Option<MeshData>,
    pub bounding_box: Option<Box3>,
}

impl Mesh {
    pub fn new(graphics: &mut Graphics, mesh_data: MeshData) -> Self {
        let gpu_mesh = graphics.new_gpu_mesh(&mesh_data).unwrap();
        let bounding_box = Box3::from_points(mesh_data.positions.iter().copied());
        Mesh {
            gpu_mesh: Some(gpu_mesh),
            mesh_data: Some(mesh_data),
            bounding_box: Some(bounding_box),
        }
    }

    pub fn recalculate_bounding_box(&mut self) {
        self.bounding_box = self
            .mesh_data
            .as_ref()
            .map(|mesh_data| Box3::from_points(mesh_data.positions.iter().copied()));
    }

    pub fn update_mesh_on_gpu(&mut self, graphics: &mut Graphics) {
        if let Some(gpu_mesh) = self.gpu_mesh.take() {
            graphics.delete_gpu_mesh(gpu_mesh)
        }
        if let Some(mesh_data) = self.mesh_data.as_ref() {
            self.gpu_mesh = Some(graphics.new_gpu_mesh(mesh_data).unwrap())
        }
    }
}

#[derive(Clone, Debug, Component, Default)]
pub struct MeshData {
    pub positions: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    pub normals: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    /// Colors are linear sRGB
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

impl AssetTrait for Mesh {
    type AssetLoader = ();
}
