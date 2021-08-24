use kserde::*;

struct Person {
    name: String,
    age: i64,
}

trait TestTrait {}

impl<S: Serializer> Serialize<S> for Person {
    fn serialize(&self, serializer: &mut S) {
        serializer.begin_object();
        serializer.property("name", &self.name);
        serializer.property("age", &self.age);
        serializer.end_object();
    }
}

impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for Person {
    fn deserialize(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;

        let mut name: Option<String> = None;
        let mut age = None;

        while let Some(p) = deserializer.has_property() {
            match &*p {
                "name" => name = Some(String::deserialize(deserializer)?),
                "age" => age = Some(i64::deserialize(deserializer)?),
                _ => {}
            }
        }

        Some(Self {
            name: name?,
            age: age?,
        })
    }
}

fn main() {
    let person = Person {
        name: "Odysseus".to_string(),
        age: 43,
    };

    let mut serializer = JSONSerializer::new();
    person.serialize(&mut serializer);
    println!("{}", serializer.done());
}
