use kserde::*;

#[derive(SerializeDeserialize)]
enum TestEnum {
    Thing { member: f32, second_member: f32 },
}

#[derive(SerializeDeserialize)]
pub struct RenderFlags(usize);

fn main() {
    let mut serializer = JSONSerializer::new();
    serializer.serialize(&TestEnum::Thing {
        member: 10.,
        second_member: 100.,
    });
    let result = serializer.done();

    TestEnum::deserialize(&mut JSONDeserializer::new(&result)).unwrap();
}
