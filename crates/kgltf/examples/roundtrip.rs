use kgltf::*;

fn main() {
    let file = std::fs::read_to_string("models/cube/Cube.gltf").unwrap();
    let gltf = GlTf::from_json(&file).unwrap();
    let gltf_json = gltf.to_json();

    let gltf = GlTf::from_json(&gltf_json).unwrap();
    println!("{:#?}", gltf);
}
