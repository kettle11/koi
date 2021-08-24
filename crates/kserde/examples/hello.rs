use kserde::*;
fn main() {
    let source = std::fs::read_to_string("examples/hello.json").unwrap();
    let json = Thing::from_json(&source).unwrap();
    println!("JSON: {:#?}", json);

    let mut serializer = JSONSerializer::new();
    (&[0, 2, 3]).serialize(&mut serializer);
    let result = serializer.done();
    println!("RESULT: {:?}", result);
}
