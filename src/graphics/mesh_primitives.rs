use crate::{
    graphics::{Mesh, MeshData},
    Handle, Vec2, Vec3,
};

pub fn vertical_quad() -> MeshData {
    let positions = vec![
        // First face
        [-0.5, -0.5, 0.0].into(),
        [0.5, -0.5, 0.0].into(),
        [0.5, 0.5, 0.0].into(),
        [-0.5, 0.5, 0.0].into(),
    ];

    let texture_coordinates = vec![
        [0.0, 1.0].into(),
        [1.0, 1.0].into(),
        [1.0, 0.0].into(),
        [0.0, 0.0].into(),
    ];

    let normals = vec![
        // 0
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
    ];

    let indices = vec![
        // First face
        [0, 1, 2],
        [0, 2, 3],
        // Second face
    ];

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

pub fn plane() -> MeshData {
    let positions = vec![
        // First face
        [-0.5, 0.0, -0.5].into(),
        [-0.5, 0.0, 0.5].into(),
        [0.5, 0.0, 0.5].into(),
        [0.5, 0.0, -0.5].into(),
    ];

    let texture_coordinates = vec![
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
    ];

    let normals = vec![
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
    ];

    let indices = vec![[0, 1, 2], [0, 2, 3]];

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

pub fn cube() -> MeshData {
    // Data for a cube mesh
    let positions = vec![
        // First face
        [-0.5, -0.5, 0.5].into(),
        [0.5, -0.5, 0.5].into(),
        [0.5, 0.5, 0.5].into(),
        [-0.5, 0.5, 0.5].into(),
        // Second face
        [-0.5, -0.5, -0.5].into(),
        [-0.5, -0.5, 0.5].into(),
        [-0.5, 0.5, 0.5].into(),
        [-0.5, 0.5, -0.5].into(),
        // Third face
        [0.5, -0.5, -0.5].into(),
        [-0.5, -0.5, -0.5].into(),
        [-0.5, 0.5, -0.5].into(),
        [0.5, 0.5, -0.5].into(),
        // Fourth face
        [0.5, -0.5, 0.5].into(),
        [0.5, -0.5, -0.5].into(),
        [0.5, 0.5, -0.5].into(),
        [0.5, 0.5, 0.5].into(),
        // Top Face
        [-0.5, 0.5, -0.5].into(),
        [-0.5, 0.5, 0.5].into(),
        [0.5, 0.5, 0.5].into(),
        [0.5, 0.5, -0.5].into(),
        // Bottom face
        [-0.5, -0.5, 0.5].into(),
        [-0.5, -0.5, -0.5].into(),
        [0.5, -0.5, -0.5].into(),
        [0.5, -0.5, 0.5].into(),
    ];

    let texture_coordinates = vec![
        // 0
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 1
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 2
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 3
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 4
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 5
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
    ];

    let normals = vec![
        // 0
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        //
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        //
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        //
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        //
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        //
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
    ];

    let indices = vec![
        // First face
        [0, 1, 2],
        [0, 2, 3],
        // Second face
        [4, 5, 6],
        [4, 6, 7],
        // Third face
        [8, 9, 10],
        [8, 10, 11],
        // Fourth face
        [12, 13, 14],
        [12, 14, 15],
        // Fifth face
        [16, 17, 18],
        [16, 18, 19],
        // Sixth face
        [20, 21, 22],
        [20, 22, 23],
    ];

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

/// A cube used for cube-map rendering. 2.0 on each dimension and inverted.
pub fn cube_map_cube() -> MeshData {
    // Data for a cube mesh
    let positions = vec![
        // First face
        [-1.0, -1.0, 1.0].into(),
        [1.0, -1.0, 1.0].into(),
        [1.0, 1.0, 1.0].into(),
        [-1.0, 1.0, 1.0].into(),
        // Second face
        [-1.0, -1.0, -1.0].into(),
        [-1.0, -1.0, 1.0].into(),
        [-1.0, 1.0, 1.0].into(),
        [-1.0, 1.0, -1.0].into(),
        // Third face
        [1.0, -1.0, -1.0].into(),
        [-1.0, -1.0, -1.0].into(),
        [-1.0, 1.0, -1.0].into(),
        [1.0, 1.0, -1.0].into(),
        // Fourth face
        [1.0, -1.0, 1.0].into(),
        [1.0, -1.0, -1.0].into(),
        [1.0, 1.0, -1.0].into(),
        [1.0, 1.0, 1.0].into(),
        // Top Face
        [-1.0, 1.0, -1.0].into(),
        [-1.0, 1.0, 1.0].into(),
        [1.0, 1.0, 1.0].into(),
        [1.0, 1.0, -1.0].into(),
        // Bottom face
        [-1.0, -1.0, 1.0].into(),
        [-1.0, -1.0, -1.0].into(),
        [1.0, -1.0, -1.0].into(),
        [1.0, -1.0, 1.0].into(),
    ];

    let texture_coordinates = vec![
        // 0
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 1
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 2
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 3
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 4
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
        // 5
        [0.0, 0.0].into(),
        [1.0, 0.0].into(),
        [1.0, 1.0].into(),
        [0.0, 1.0].into(),
    ];

    let normals = vec![
        // 0
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        //
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        [-1.0, 0.0, 0.0].into(),
        //
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        [0.0, 0.0, -1.0].into(),
        //
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        [1.0, 0.0, 0.0].into(),
        //
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        //
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
        [0.0, -1.0, 0.0].into(),
    ];

    let indices = vec![
        // First face
        [2, 1, 0],
        [3, 2, 0],
        // Second face
        [6, 5, 4],
        [7, 6, 4],
        // Third face
        [10, 9, 8],
        [11, 10, 8],
        // Fourth face
        [14, 13, 12],
        [15, 14, 12],
        // Fifth face
        [18, 17, 16],
        [19, 18, 16],
        // Sixth face
        [22, 21, 20],
        [23, 22, 20],
    ];

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

/// A triangle for debugging purposes
pub fn triangle() -> MeshData {
    let positions = vec![
        [0.0, 1.0, 0.0].into(),
        [1.0, -1.0, 0.0].into(),
        [-1.0, -1.0, 0.0].into(),
    ];

    let texture_coordinates = vec![[1.0, 1.0].into(), [0.5, 1.0].into(), [0.0, 0.0].into()];

    let normals = vec![
        // 0
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
        [0.0, 0.0, 1.0].into(),
    ];

    let indices = vec![[0, 1, 2]];

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

/// Creates a ring
/// Thickness is the radius of the tube itself.
pub fn ring(
    tube_radius: f32,
    ring_radius: f32,
    tube_resolution: usize,
    ring_resolution: usize,
) -> MeshData {
    let mut tube_points = Vec::with_capacity(tube_resolution);
    let mut angle: f32 = 0.;
    let increment = (2.0 * std::f32::consts::PI) / (tube_resolution) as f32;

    for _ in 0..tube_resolution {
        let (sin, cos) = angle.sin_cos();
        let position = Vec3::new(cos, sin, 0.) * tube_radius;
        tube_points.push(position);
        angle += increment;
    }

    revolve(&tube_points, ring_radius, ring_resolution)
}

/// Revolves a polygon (centered at zero) to make a loop.
fn revolve(polygon: &[Vec3], radius: f32, resolution: usize) -> MeshData {
    let polygon_len = polygon.len();

    let len = resolution * polygon_len;
    let mut positions = Vec::with_capacity(len);
    let mut normals = Vec::with_capacity(len);
    let mut uvs = Vec::with_capacity(len);
    let mut indices = Vec::with_capacity(resolution * 3 * 2 * polygon_len);

    let increment = -(2.0 * std::f32::consts::PI) / (resolution) as f32;
    let mut angle: f32 = 0.;

    let uv_increment_x = 1.0 / resolution as f32;
    let uv_increment_y = 1.0 / polygon.len() as f32;

    let mut uv_x = 0.;

    for _ in 0..resolution {
        let mut uv_y = 0.;

        let (sin, cos) = angle.sin_cos();
        let center_offset = Vec3::new(cos, 0., sin) * radius;
        for v in polygon.iter() {
            let vertex_direction = Vec3::new(v[0] * cos, v[1], v[0] * sin);
            positions.push(vertex_direction + center_offset);
            normals.push(vertex_direction.normalized());
            uvs.push(Vec2::new(uv_x, uv_y));

            uv_y += uv_increment_y;
        }
        uv_x += uv_increment_x;
        angle += increment;
    }

    let polygon_len = polygon_len as u32;
    let mut previous_start = (polygon_len) * (resolution as u32 - 1);
    for k in 0..resolution {
        let current_start = k as u32 * polygon_len;
        let mut first = polygon_len - 1;
        for second in 0..polygon_len {
            indices.push([
                previous_start + first,
                current_start + first,
                current_start + second,
            ]);
            indices.push([
                previous_start + first,
                current_start + second,
                previous_start + second,
            ]);
            first = second;
        }
        previous_start = current_start;
    }

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates: uvs,
        ..Default::default()
    }
}

pub fn cone(radius: f32, height: f32, resolution: usize) -> MeshData {
    let mut positions = Vec::with_capacity(resolution + 1);
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());

    let mut indices: Vec<[u32; 3]> = Vec::with_capacity(resolution * 3);

    let mut angle: f32 = 0.;
    let increment = (2.0 * std::f32::consts::PI) / (resolution) as f32;

    // The top point of the cone
    positions.push(Vec3::Y * height);
    normals.push(Vec3::Y);
    uvs.push(Vec2::new(0.5, 0.5));

    for _ in 0..resolution {
        let (sin, cos) = angle.sin_cos();
        let direction = Vec3::X * cos * radius + Vec3::Z * sin * radius;
        let position = direction * radius;
        positions.push(position);
        normals.push(direction.normalized());
        uvs.push(Vec2::new(0.5, 0.5));

        angle += increment;
    }

    // Sides
    for i in 1..resolution {
        indices.push([i as u32, i as u32 + 1, 0]);
    }
    indices.push([positions.len() as u32 - 1, 1, 0]);

    // Bottom
    for i in 2..resolution {
        indices.push([i as u32, i as u32 + 1, 1]);
    }

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates: uvs,
        ..Default::default()
    }
}

pub fn uv_sphere(horizontal_segments: u32, vertical_segments: u32, uv_scale: Vec2) -> MeshData {
    use std::f32::consts::PI;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for y in 0..=vertical_segments {
        let y_segment = (y as f32) / (vertical_segments as f32);
        let y = -(y_segment * PI).cos();
        for x in 0..=horizontal_segments {
            let x_segment = (x as f32) / (horizontal_segments as f32);

            let x = (x_segment * 2.0 * PI).cos() * (y_segment * PI).sin();
            let z = (x_segment * 2.0 * PI).sin() * (y_segment * PI).sin();

            // Multiply by 0.5 to make the sphere's radius 0.5 by default
            positions.push(Vec3::new(x, y, z) * 0.5);
            normals.push(Vec3::new(x, y, z).normalized());
            uvs.push(Vec2::new(x_segment * uv_scale[0], y_segment * uv_scale[1]));
        }
    }

    let mut indices = Vec::new();
    for y in 0..vertical_segments {
        for x in 0..horizontal_segments {
            indices.push([
                y * (horizontal_segments + 1) + x,
                (y + 1) * (horizontal_segments + 1) + x,
                y * (horizontal_segments + 1) + x + 1,
            ]);
            indices.push([
                (y + 1) * (horizontal_segments + 1) + x,
                (y + 1) * (horizontal_segments + 1) + x + 1,
                y * (horizontal_segments + 1) + x + 1,
            ]);
        }
    }

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates: uvs,
        ..Default::default()
    }
}

/// All merging meshes must have normals and texture coordinates
/// The second argument is a slice of tuples. The first value in the tuple is an index to a mesh in the first slice.
pub fn merge_meshes(
    meshes: &[MeshData],
    index_and_transform: &[(usize, crate::Transform)],
) -> MeshData {
    let (vertex_count, index_count) =
        index_and_transform
            .iter()
            .fold((0, 0), |(vertex_count, index_count), (index, _)| {
                (
                    vertex_count + meshes[*index].positions.len(),
                    index_count + meshes[*index].indices.len(),
                )
            });
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut texture_coordinates: Vec<Vec2> = Vec::with_capacity(vertex_count);
    let mut indices = Vec::with_capacity(index_count);

    for (index, transform) in index_and_transform.iter() {
        let model = transform.model();
        let mesh = &meshes[*index];
        let offset = positions.len() as u32;
        for p in mesh.positions.iter() {
            positions.push(model.transform_point(*p));
        }
        for n in &mesh.normals {
            // This may be an incorrect way to transform the normal
            normals.push(model.transform_vector(*n));
        }

        texture_coordinates.extend(&mesh.texture_coordinates);

        for i in mesh.indices.iter() {
            indices.push([i[0] + offset, i[1] + offset, i[2] + offset]);
        }
    }

    MeshData {
        positions,
        indices,
        normals,
        texture_coordinates,
        ..Default::default()
    }
}

impl Mesh {
    pub const PLANE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(1);
    pub const VERTICAL_QUAD: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(2);
    pub const CUBE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(3);
    pub const SPHERE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(4);
    pub const RING: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(5);
    pub const TRIANGLE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(6);
    pub const CONE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(7);
    pub const CUBE_MAP_CUBE: Handle<Mesh> = Handle::<Mesh>::new_with_just_index(8);
}

pub(crate) fn initialize_static_primitives(
    graphics: &mut crate::Graphics,
    meshes: &mut crate::Assets<Mesh>,
) {
    let mesh_data = plane();
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::PLANE);
    let mesh_data = vertical_quad();
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::VERTICAL_QUAD);

    let mesh_data = cube();
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::CUBE);
    let mesh_data = uv_sphere(32, 32, Vec2::ONE);
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::SPHERE);
    let mesh_data = ring(0.1, 1.0, 8, 20);
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::RING);
    let mesh_data = triangle();
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::TRIANGLE);
    let mesh_data = cone(0.7, 1.0, 20);
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::CONE);
    let mesh_data = cube_map_cube();
    meshes.add_and_leak(Mesh::new(graphics, mesh_data), &Mesh::CUBE_MAP_CUBE);
}
