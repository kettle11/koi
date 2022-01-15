use kserde::*;

#[derive(SerializeDeserialize)]
enum TestEnum {
    Thing { member: f32, second_member: f32 },
}

/*
impl<S: Serializer> Serialize<S> for TestEnum {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        match self {
            Self::Thing(v0, v1, v2) => serializer.property("Thing", &(v0, v1, v2)),
        }
        serializer.end_object();
    }
}
*/
fn main() {
    /*
    let source = std::fs::read_to_string("examples/hello.json").unwrap();
    let json = Thing::from_json(&source).unwrap();
    println!("JSON: {:#?}", json);

    let mut serializer = JSONSerializer::new();
    (&[0, 2, 3]).serialize(&mut serializer);
    let result = serializer.done();
    println!("RESULT: {:?}", result);
    */

    let mut serializer = JSONSerializer::new();
    serializer.serialize(&TestEnum::Thing {
        member: 10.,
        second_member: 100.,
    });
    let result = serializer.done();
    println!("RESULT: {}", result);

    TestEnum::deserialize(&mut JSONDeserializer::new(&result)).unwrap();
}
