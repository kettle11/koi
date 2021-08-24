use kgltf::*;

fn main() {
    let bytes = std::fs::read("models/fox.glb").unwrap();
    let glb = GLB::from_bytes(&bytes).unwrap();
    println!("GLB: {:#?}", glb);
}
