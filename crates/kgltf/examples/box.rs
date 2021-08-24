use kgltf::*;

fn main() {
    let file = std::fs::read_to_string("models/box/Box.gltf").unwrap();
    let gltf = GlTf::from_json(&file);
    println!("{:#?}", gltf);
}
