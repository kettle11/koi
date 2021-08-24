//! The code in this file is inefficient and quite spaghetti-ish, but
//! it only has to run to generate the library.

use kserde::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::path::Path;

#[derive(Debug)]
struct Property {
    name: String,
    required: bool,
    schema: Schema,
    /// If these other values are defined, this cannot be defined.
    incompatible_with: Vec<String>,
}

// For now only string and number enum values are supported.
#[derive(Debug)]

enum EnumValue {
    String(String),
    Number(f32),
}

#[derive(Debug)]
enum SchemaType {
    String,
    Integer {
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
    },
    Number {
        minimum: Option<f32>,
        maximum: Option<f32>,
        multiple_of: Option<f32>,
    },
    Object {
        properties: Vec<Box<Property>>,
        properties_by_name: HashMap<String, usize>,
        additional_properties: Vec<Box<Schema>>,
        min_properties: Option<u32>,
        max_properties: Option<u32>,
    },
    Array {
        min_items: Option<u32>,
        max_items: Option<u32>,
        items: Option<Box<Schema>>,
    },
    Enum,
    // AllOf is baked in.
    // Not is ignored for now.
    Boolean,
    Any,
}

#[derive(Debug)]
struct Schema {
    schema_type: SchemaType,
    title: Option<String>,
    description: Option<String>,
    default: Option<ThingOwned>,
    any_of: Vec<Schema>,
    one_of: Vec<Schema>,
    /// Only filled out if this is an enum
    enum_values: Vec<EnumValue>,
}

struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    fn extend_from_schema(&mut self, schema: &mut Schema, thing: &Thing) {
        let thing = thing.object().unwrap();

        if let Some(ref_) = thing.get("$ref") {
            let ref_ = ref_.item.string().unwrap();
            let source = std::fs::read_to_string(Path::new("schema").join(&**ref_))
                .expect("Could not find file");
            let json = Thing::from_json(&source).expect("Could not parse JSON");
            self.extend_from_schema(schema, &json);
            return;
        }

        let title = thing
            .get("title")
            .map(|s| s.item.string().unwrap().to_string());
        let description = thing
            .get("description")
            .map(|s| s.item.string().unwrap().to_string());
        if schema.title.is_none() {
            schema.title = title;
        }

        if let Some(default) = thing.get("default") {
            schema.default = Some(default.item.to_owned());
        }

        if schema.description.is_none() {
            schema.description = description;
        }

        let type_ = thing.get("type").map(|t| t.item.string().unwrap());
        match type_ {
            Some(c) => match &**c {
                "boolean" => match schema.schema_type {
                    SchemaType::Boolean => {}
                    SchemaType::Any => {
                        schema.schema_type = SchemaType::Boolean;
                    }
                    _ => panic!("Type already set"),
                },
                "string" => match schema.schema_type {
                    SchemaType::String => {}
                    SchemaType::Any => {
                        schema.schema_type = SchemaType::String;
                    }
                    _ => panic!("Type already set"),
                },
                "integer" => {
                    match schema.schema_type {
                        SchemaType::Integer { .. } => {}
                        SchemaType::Any => {
                            schema.schema_type = SchemaType::Integer {
                                minimum: None,
                                maximum: None,
                                multiple_of: None,
                            }
                        }
                        _ => panic!("Type already set"),
                    }
                    let new_minimum = thing
                        .get("minimum")
                        .map(|n| n.item.number().unwrap() as i64);
                    let new_maximum = thing
                        .get("maximum")
                        .map(|n| n.item.number().unwrap() as i64);
                    let new_multiple_of = thing
                        .get("multipleOf")
                        .map(|n| n.item.number().unwrap() as i64);

                    match &mut schema.schema_type {
                        SchemaType::Integer {
                            minimum,
                            maximum,
                            multiple_of,
                        } => {
                            *minimum = combine_option(*minimum, new_minimum);
                            *maximum = combine_option(*maximum, new_maximum);
                            *multiple_of = combine_option(*multiple_of, new_multiple_of);
                        }
                        _ => unreachable!(),
                    };
                }
                "number" => {
                    match schema.schema_type {
                        SchemaType::Number { .. } => {}
                        SchemaType::Any => {
                            schema.schema_type = SchemaType::Number {
                                minimum: None,
                                maximum: None,
                                multiple_of: None,
                            }
                        }
                        _ => panic!("Type already set"),
                    }
                    let new_minimum = thing
                        .get("minimum")
                        .map(|n| n.item.number().unwrap() as f32);
                    let new_maximum = thing
                        .get("maximum")
                        .map(|n| n.item.number().unwrap() as f32);
                    let new_multiple_of = thing
                        .get("multipleOf")
                        .map(|n| n.item.number().unwrap() as f32);

                    match &mut schema.schema_type {
                        SchemaType::Number {
                            minimum,
                            maximum,
                            multiple_of,
                        } => {
                            *minimum = combine_option(*minimum, new_minimum);
                            *maximum = combine_option(*maximum, new_maximum);
                            *multiple_of = combine_option(*multiple_of, new_multiple_of);
                        }
                        _ => unreachable!(),
                    };
                }
                "object" => {
                    match schema.schema_type {
                        SchemaType::Object { .. } => {}
                        SchemaType::Any => {
                            schema.schema_type = SchemaType::Object {
                                properties: Vec::new(),
                                properties_by_name: HashMap::new(),
                                additional_properties: Vec::new(),
                                min_properties: None,
                                max_properties: None,
                            }
                        }
                        _ => panic!("Type already set"),
                    }

                    match &mut schema.schema_type {
                        SchemaType::Object {
                            min_properties,
                            max_properties,
                            properties,
                            properties_by_name,
                            additional_properties,
                        } => {
                            let new_min_properties = thing
                                .get("minProperties")
                                .map(|n| n.item.number().unwrap() as u32);
                            let new_max_properties = thing
                                .get("maxProperties")
                                .map(|n| n.item.number().unwrap() as u32);

                            *min_properties = combine_option(*min_properties, new_min_properties);
                            *max_properties = combine_option(*max_properties, new_max_properties);

                            if let Some(new_additional_properties) =
                                thing.get("additionalProperties")
                            {
                                let schema = self.parse_schema(&new_additional_properties.item);
                                additional_properties.push(Box::new(schema));
                            }

                            if let Some(new_properties) = thing.get("properties") {
                                let mut required = HashSet::new();
                                if let Some(required_array) = thing.get("required") {
                                    let required_array = required_array.item.array().unwrap();
                                    for i in required_array {
                                        required.insert(i.string().unwrap());
                                    }
                                }

                                let new_properties = new_properties.item.object().unwrap();
                                let mut new_properties: Vec<_> =
                                    new_properties.into_iter().collect();
                                new_properties.sort_by_key(|(_, i)| i.index);

                                for (key, value) in new_properties {
                                    let property = Property {
                                        name: key.to_string(),
                                        required: required.contains(key),
                                        schema: self.parse_schema(&value.item),
                                        incompatible_with: Vec::new(),
                                    };

                                    if let Some(index) = properties_by_name.get(&**key) {
                                        match properties[*index].schema.schema_type {
                                            SchemaType::Any => {
                                                properties[*index] = Box::new(property)
                                            }
                                            _ => {}
                                        }
                                        // println!("ALREADY CONTAINS: {:?}", key);
                                    } else {
                                        properties_by_name
                                            .insert(key.to_string(), properties.len());
                                        properties.push(Box::new(property));
                                    }
                                }
                            }

                            if let Some(not) = thing.get("not") {
                                let object = not.item.object().unwrap();
                                if let Some(required) = object.get("required") {
                                    let required = required.item.array().unwrap();
                                    let first_required = required[0].string().unwrap().to_string();
                                    let second_required = required[1].string().unwrap().to_string();
                                    properties[*properties_by_name.get(&first_required).unwrap()]
                                        .incompatible_with
                                        .push(second_required.to_string());
                                    properties[*properties_by_name.get(&second_required).unwrap()]
                                        .incompatible_with
                                        .push(first_required.to_string());
                                    // These items will be pairs.
                                }

                                if let Some(any_of) = object.get("anyOf") {
                                    let any_of = any_of.item.array().unwrap();
                                    for object in any_of {
                                        // This code is mostly copy-pasted from above
                                        let required =
                                            object.object().unwrap().get("required").unwrap();
                                        let required = required.item.array().unwrap();
                                        let first_required =
                                            required[0].string().unwrap().to_string();
                                        let second_required =
                                            required[1].string().unwrap().to_string();
                                        properties
                                            [*properties_by_name.get(&first_required).unwrap()]
                                        .incompatible_with
                                        .push(second_required.to_string());
                                        properties
                                            [*properties_by_name.get(&second_required).unwrap()]
                                        .incompatible_with
                                        .push(first_required.to_string());

                                        println!(
                                            "INCOMPATIBLE WITH: {}, {}",
                                            first_required, second_required
                                        );
                                        // These items will be pairs.
                                    }
                                }
                            }
                            // Todo: Additional properties
                        }
                        _ => unreachable!(),
                    };
                }
                "array" => {
                    match schema.schema_type {
                        SchemaType::Array { .. } => {}
                        SchemaType::Any => {
                            schema.schema_type = SchemaType::Array {
                                min_items: None,
                                max_items: None,
                                items: None,
                            }
                        }
                        _ => panic!("Type already set"),
                    }
                    let new_min_items = thing
                        .get("minItems")
                        .map(|n| n.item.number().unwrap() as u32);
                    let new_max_items = thing
                        .get("maxItems")
                        .map(|n| n.item.number().unwrap() as u32);

                    match &mut schema.schema_type {
                        SchemaType::Array {
                            min_items,
                            max_items,
                            items,
                            ..
                        } => {
                            *min_items = combine_option(*min_items, new_min_items);
                            *max_items = combine_option(*max_items, new_max_items);
                            let new_items = thing.get("items").unwrap();
                            let new_items = self.parse_schema(&new_items.item);
                            *items = Some(Box::new(new_items));
                        }
                        _ => unreachable!(),
                    };
                }
                _ => panic!("Unknown schema type: {}", c),
            },
            None => (),
        };

        if let Some(e) = thing.get("enum") {
            let e = e.item.array().unwrap();
            for member in e.iter() {
                let member = match member {
                    Thing::String(s) => EnumValue::String(s.to_string()),
                    Thing::Number(n) => EnumValue::Number(*n as f32),
                    _ => panic!("Unsupported enum type"),
                };
                schema.enum_values.push(member)
            }
            schema.schema_type = SchemaType::Enum;
        }

        if let Some(all_of) = thing.get("allOf") {
            let all_of = all_of.item.array().unwrap();
            for extend_with in all_of {
                self.extend_from_schema(schema, extend_with);
            }
        }

        if let Some(any_of) = thing.get("anyOf") {
            let any_of = any_of.item.array().unwrap();
            for extend_with in any_of {
                schema.any_of.push(self.parse_schema(extend_with));
            }
        }

        if let Some(one_of) = thing.get("oneOf") {
            let one_of = one_of.item.array().unwrap();
            for extend_with in one_of {
                schema.one_of.push(self.parse_schema(extend_with));
            }
        }
    }

    pub fn parse_schema(&mut self, thing: &Thing) -> Schema {
        let mut schema = Schema {
            schema_type: SchemaType::Any,
            title: None,
            description: None,
            default: None,
            one_of: Vec::new(),
            any_of: Vec::new(),
            enum_values: Vec::new(),
        };

        self.extend_from_schema(&mut schema, thing);

        schema
    }
}

fn combine_option<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    if a.is_some() {
        a
    } else {
        b
    }
}

use heck::{CamelCase, SnakeCase};
#[derive(Clone)]
struct RustStructProperty {
    name: String,
    json_name: String,
    property_type: RustType,
    description: Option<String>,
    default_value: Option<ThingOwned>,
    incompatible_with: Vec<String>,
    optional: bool,
}

#[derive(Clone)]
struct RustStruct {
    name: String,
    json_name: Option<String>,
    description: String,
    properties: Vec<RustStructProperty>,
}

#[derive(Clone)]
enum JsonEnumValue {
    String(String),
    Integer(i32),
}

#[derive(Clone)]
struct EnumMember {
    name: String,
    json_value: JsonEnumValue,
    description: Option<String>,
    value: Option<u32>,
}
#[derive(Clone)]
struct RustEnum {
    name: String,
    json_name: Option<String>,
    description: String,
    members: Vec<EnumMember>,
}

#[derive(Clone)]
enum RustType {
    String,
    USIZE,
    Boolean,
    F32,
    Struct(RustStruct),
    Array(usize, Box<RustType>),
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    Option(Box<RustType>), // Need an enum variant.
    //Enum { name: String, members: Vec<String> },
    Enum(RustEnum),
    KSerdeOwnedThing,
    Unimplemented,
}

impl RustType {
    fn type_name(&self) -> String {
        match self {
            RustType::String => "String".to_string(),
            RustType::USIZE => "usize".to_string(),
            RustType::Boolean => "bool".to_string(),
            RustType::F32 => "f32".to_string(),
            RustType::Struct(s) => s.name.clone(),
            RustType::Array(size, inner_type) => {
                format!("[{}; {:?}]", inner_type.type_name(), size)
            }
            RustType::Vec(inner_type) => {
                format!("Vec<{}>", inner_type.type_name())
            }
            RustType::Option(inner_type) => {
                format!("Option<{}>", inner_type.type_name())
            }
            RustType::HashMap(key_type, value_type) => {
                format!(
                    "HashMap<{}, {}>",
                    key_type.type_name(),
                    value_type.type_name()
                )
            }
            RustType::Enum(s) => s.name.clone(),
            RustType::KSerdeOwnedThing => "ThingOwned".to_string(),
            RustType::Unimplemented => "UNIMPLEMENTED".to_string(),
        }
    }
}

struct RustGenerator {
    rust_types: Vec<(String, RustType)>,
    rust_types_set: HashSet<String>,
}

impl<'a> RustGenerator {
    pub fn new() -> Self {
        Self {
            rust_types: Vec::new(),
            rust_types_set: HashSet::new(),
        }
    }

    fn rust_type_from_schema(&mut self, enum_name: &str, schema: &'a Schema) -> RustType {
        match &schema.schema_type {
            SchemaType::String => RustType::String,
            SchemaType::Boolean => RustType::Boolean,
            SchemaType::Integer { .. } => RustType::USIZE,
            SchemaType::Number { .. } => RustType::F32,
            SchemaType::Object {
                properties,
                additional_properties,
                ..
            } => {
                if let Some(title) = schema.title.as_ref() {
                    let name = title.to_camel_case();
                    let json_name = schema.title.clone();
                    // let name = json_name.to_camel_case();
                    let description = schema.description.clone().unwrap();
                    let mut struct_properties = Vec::new();

                    if let Some(additional_property) = additional_properties.get(0) {
                        match additional_property.schema_type {
                            SchemaType::Integer { .. } => RustType::HashMap(
                                Box::new(RustType::String),
                                Box::new(RustType::USIZE),
                            ),
                            SchemaType::Object { .. } => RustType::HashMap(
                                Box::new(RustType::String),
                                Box::new(RustType::KSerdeOwnedThing),
                            ),
                            _ => panic!("Unsupported schema type for HashMap"),
                        }
                    } else {
                        for property in properties {
                            let mut enum_name = name.clone();
                            enum_name.push_str(&property.name.to_camel_case());

                            let mut property_name = property.name.to_snake_case();
                            enum_name.to_camel_case();
                            let property_type =
                                self.rust_type_from_schema(&&enum_name, &property.schema);

                            // If a property is not required remap it to an `Option`, unless
                            let property_type = if !property.required
                                && (property.schema.default.is_none()
                                    || property.incompatible_with.len() > 0)
                            {
                                match property_type {
                                    //  RustType::Vec(..) | RustType::HashMap(..) => property_type,
                                    _ => RustType::Option(Box::new(property_type)),
                                }
                            } else {
                                property_type
                            };

                            if property_name == "type" {
                                property_name.push('_');
                            }

                            struct_properties.push(RustStructProperty {
                                name: property_name,
                                json_name: property.name.clone(),
                                description: property.schema.description.clone(),
                                optional: !property.required,
                                default_value: property.schema.default.clone(),
                                incompatible_with: property.incompatible_with.clone(),
                                property_type,
                            })
                        }
                        let rust_type = RustType::Struct(RustStruct {
                            name: name.clone(),
                            json_name,
                            description,
                            properties: struct_properties,
                        });
                        if self.rust_types_set.insert(name.clone()) {
                            self.rust_types.push((name, rust_type.clone()));
                        }
                        rust_type
                    }
                } else {
                    // This is probably just a HashMap
                    if let Some(p) = additional_properties.get(0) {
                        match p.schema_type {
                            SchemaType::Integer { .. } => RustType::HashMap(
                                Box::new(RustType::String),
                                Box::new(RustType::USIZE),
                            ),
                            SchemaType::Object { .. } => RustType::HashMap(
                                Box::new(RustType::String),
                                Box::new(RustType::KSerdeOwnedThing),
                            ),
                            _ => panic!("Unsupported schema type for HashMap"),
                        }
                    } else {
                        panic!("Expected schema with additional properties")
                    }
                }
            }
            SchemaType::Array {
                min_items,
                max_items,
                items,
            } => {
                let item_schema = self.rust_type_from_schema(&enum_name, items.as_ref().unwrap());
                if min_items.is_some() && min_items == max_items {
                    let size = min_items.unwrap();
                    RustType::Array(size as usize, Box::new(item_schema))
                } else {
                    RustType::Vec(Box::new(item_schema))
                }
            }
            SchemaType::Any => {
                if schema.any_of.len() > 0 {
                    // let name = enum_name.to_camel_case();
                    let json_name = schema.title.clone();
                    let description = schema.description.clone().unwrap();

                    let mut members = Vec::new();
                    for value in schema.any_of.iter() {
                        match value.schema_type {
                            SchemaType::Enum => {
                                members.push(match &value.enum_values[0] {
                                    EnumValue::Number(n) => EnumMember {
                                        name: value.description.clone().unwrap().to_camel_case(),
                                        description: None,
                                        json_value: JsonEnumValue::Integer(*n as i32),
                                        value: Some(*n as u32),
                                    },
                                    EnumValue::String(s) => EnumMember {
                                        name: s.clone().to_camel_case(),
                                        description: value.description.clone(),
                                        json_value: JsonEnumValue::String(s.clone()),
                                        value: None,
                                    },
                                });
                            }
                            _ => {}
                        }
                    }
                    let rust_type = RustType::Enum(RustEnum {
                        name: enum_name.to_string(),
                        json_name,
                        description,
                        members,
                    });

                    if self.rust_types_set.insert(enum_name.to_string()) {
                        self.rust_types
                            .push((enum_name.to_string(), rust_type.clone()));
                    }
                    rust_type
                } else {
                    // This is an extras object.
                    //println!("UNIMPLEMENTED HERE: {:#?}", schema.title);
                    RustType::KSerdeOwnedThing
                }
            }
            _ => todo!(),
        }
    }

    pub fn generate_struct(&mut self, name: String, schema: &'a Schema) {
        self.rust_type_from_schema(&name, schema);
    }

    pub fn generate(&mut self, schema: &'a Schema) -> String {
        self.generate_struct("".to_string(), schema);

        let mut output = String::new();
        write!(output, "use kserde::*;\n\n").unwrap();
        write!(output, "use std::collections::HashMap;\n\n").unwrap();

        // Reverse because we want to the top level structure at the top of the file
        for (_, s) in self.rust_types.iter().rev() {
            match s {
                RustType::Struct(s) => {
                    write!(output, "/// {}\n", s.description).unwrap();
                    write!(
                        output,
                        "#[derive(Debug, Clone)]\npub struct {} {{\n",
                        s.name
                    )
                    .unwrap();
                    for property in s.properties.iter() {
                        if let Some(description) = &property.description {
                            write!(output, "    /// {}\n", description).unwrap();
                        }
                        match &property.property_type {
                            RustType::Option(r) => {
                                match &**r {
                                    // In this case we won't use an option, we'll just leave the data structure empty.
                                    RustType::Vec(..) | RustType::HashMap(..) => {
                                        write!(
                                            output,
                                            "    pub {}: {},\n",
                                            property.name,
                                            r.type_name()
                                        )
                                        .unwrap();
                                        continue;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        };
                        // Default case
                        write!(
                            output,
                            "    pub {}: {},\n",
                            property.name,
                            property.property_type.type_name()
                        )
                        .unwrap();
                    }
                    write!(output, "}}\n\n").unwrap();

                    // Implement serialization for this type
                    write!(
                        output,
                        "impl<S: Serializer> Serialize<S> for {} {{\n",
                        s.name
                    )
                    .unwrap();
                    write!(output, "    fn serialize(&self, serializer: &mut S) {{\n").unwrap();
                    write!(output, "        serializer.begin_object();\n").unwrap();
                    for property in s.properties.iter() {
                        if property.optional
                            && (property.default_value.is_none()
                                && property.incompatible_with.len() > 0)
                        {
                            match &property.property_type {
                                // Only serialize if the Vec is not empty.
                                RustType::Vec(_) => {
                                    write!(
                                        output,
                                        "        if !self.{}.is_empty() {{\n",
                                        property.name
                                    )
                                    .unwrap();
                                    write!(
                                        output,
                                        "           serializer.property(\"{}\", &self.{});\n",
                                        property.json_name, property.name
                                    )
                                    .unwrap();
                                    write!(output, "        }}\n").unwrap();
                                }
                                // Only serialize if the option is not empty.
                                RustType::Option(inner) => {
                                    match &**inner {
                                        // Only serialize if the Vec is not empty.
                                        RustType::Vec(_) => {
                                            write!(
                                                output,
                                                "        if !self.{}.is_empty() {{\n",
                                                property.name
                                            )
                                            .unwrap();
                                            write!(
                                                output,
                                                "           serializer.property(\"{}\", &self.{});\n",
                                                property.json_name, property.name
                                            )
                                            .unwrap();
                                            write!(output, "        }}\n").unwrap();
                                        }
                                        RustType::HashMap(..) => {
                                            write!(
                                                output,
                                                "        if !self.{}.is_empty() {{\n",
                                                property.name
                                            )
                                            .unwrap();
                                            write!(
                                                output,
                                                "           serializer.property(\"{}\", &self.{});\n",
                                                property.json_name, property.name
                                            )
                                            .unwrap();
                                            write!(output, "        }}\n").unwrap();
                                        }
                                        _ => {
                                            write!(
                                                output,
                                                "        if let Some(v) = self.{}.as_ref() {{\n",
                                                property.name
                                            )
                                            .unwrap();
                                            write!(
                                                output,
                                                "           serializer.property(\"{}\", v);\n",
                                                property.json_name
                                            )
                                            .unwrap();
                                            write!(output, "        }}\n").unwrap();
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Unexpected optional type: {:?}", &property.name)
                                }
                            }
                        } else {
                            write!(
                                output,
                                "        serializer.property(\"{}\", &self.{});\n",
                                property.json_name, property.name
                            )
                            .unwrap();
                        }
                    }
                    write!(output, "        serializer.end_object();\n").unwrap();

                    write!(output, "    }}\n").unwrap();
                    write!(output, "}}\n").unwrap();

                    // Implement deserialization for this type.
                    write!(
                        output,
                        "impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for {} {{\n",
                        s.name
                    )
                    .unwrap();
                    write!(
                        output,
                        "    fn deserialize(deserializer: &mut D) -> Option<Self> {{\n"
                    )
                    .unwrap();
                    write!(
                        output,
                        "        deserializer.begin_object().then(|| {{}})?;\n"
                    )
                    .unwrap();
                    for property in s.properties.iter() {
                        write!(output, "        let mut {} = None;\n", property.name,).unwrap();
                    }

                    write!(
                        output,
                        "\n        while let Some(property) = deserializer.has_property() {{\n",
                    )
                    .unwrap();
                    {
                        write!(output, "             match &*property {{\n",).unwrap();
                        for property in s.properties.iter() {
                            write!(output, "                \"{}\" =>", property.json_name,)
                                .unwrap();

                            match &property.property_type {
                                RustType::Option(r) => {
                                    write!(
                                        output,
                                        " {} = Some(<{}>::deserialize(deserializer)?),\n",
                                        property.name,
                                        r.type_name()
                                    )
                                    .unwrap();
                                }
                                _ => {
                                    write!(
                                        output,
                                        " {} = Some(<{}>::deserialize(deserializer)?),\n",
                                        property.name,
                                        property.property_type.type_name()
                                    )
                                    .unwrap();
                                }
                            }
                        }
                        write!(output, "                _ => {{}}\n",).unwrap();
                        write!(output, "            }}\n",).unwrap();
                    }
                    write!(output, "        }}\n\n").unwrap();

                    write!(output, "        Some(Self {{\n").unwrap();
                    for property in s.properties.iter() {
                        let mut optional = false;
                        let mut value = match &property.property_type {
                            RustType::Option(r) => {
                                optional = true;
                                match &**r {
                                    // In this case we won't use an option, we'll just leave the data structure empty.
                                    RustType::Vec(..) => {
                                        write!(
                                            output,
                                            "            {}: {}.unwrap_or_else(|| Vec::new()),\n",
                                            property.name, property.name
                                        )
                                        .unwrap();
                                        continue;
                                    }
                                    RustType::HashMap(..) => {
                                        write!(
                                            output,
                                            "            {}: {}.unwrap_or_else(|| HashMap::new()),\n",
                                            property.name, property.name,
                                        )
                                        .unwrap();
                                        continue;
                                    }
                                    _ => property.name.clone(),
                                }
                            }
                            _ => property.name.clone(),
                        };

                        let mut cloned = false;
                        if let Some(default_value) = &property.default_value {
                            value = format!(
                                "{}{}.map_or_else(|| {}, |m| m)",
                                value,
                                if property.incompatible_with.len() > 0 {
                                    cloned = true;
                                    ".clone()"
                                } else {
                                    ""
                                },
                                match default_value {
                                    ThingOwned::String(s) => {
                                        match &property.property_type {
                                            // Find an enum member with the same name
                                            RustType::Enum(e) => {
                                                let mut result = "".to_string();
                                                for member in &e.members {
                                                    match &member.json_value {
                                                        JsonEnumValue::String(v) => {
                                                            if v == s {
                                                                result = format!(
                                                                    "{}::{}",
                                                                    e.name, member.name
                                                                );
                                                                break;
                                                            }
                                                        }
                                                        _ => unimplemented!(),
                                                    }
                                                }
                                                result
                                            }
                                            _ => s.clone(),
                                        }
                                    }
                                    ThingOwned::Bool(b) => b.to_string(),
                                    ThingOwned::Number(n) => {
                                        match &property.property_type {
                                            RustType::USIZE => {
                                                format!("{}usize", n.to_string())
                                            }
                                            RustType::F32 => {
                                                format!("{}f32", n.to_string())
                                            }
                                            RustType::Enum(e) => {
                                                // Find the matching enum value
                                                let mut s = "".to_string();
                                                for member in &e.members {
                                                    if *n as u32 == member.value.unwrap() {
                                                        s = format!("{}::{}", e.name, member.name);
                                                        break;
                                                    }
                                                }
                                                s
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    ThingOwned::Object(_) => unimplemented!(),
                                    ThingOwned::Array(a) => {
                                        let mut s = "[".to_string();
                                        for v in a {
                                            match v {
                                                ThingOwned::Number(n) => {
                                                    s.push_str(&format!("{}f32, ", n.to_string()))
                                                }
                                                _ => s.push_str(&format!("{}, ", &v.to_json())),
                                            }
                                        }
                                        s.push_str("]");
                                        s
                                    }
                                    ThingOwned::Null => {
                                        unimplemented!()
                                    }
                                }
                            );

                            if optional {
                                value = format!("Some({})", value)
                            }
                        }

                        if !optional && property.default_value.is_none() {
                            value += "?"
                        }

                        if property.incompatible_with.len() > 0 {
                            let mut condition = "".to_string();
                            let mut previous = false;

                            // Set this property to null if incompatible properties exist on the object.
                            // This generated code isn't entirely correct as it uses the JSON name, not the
                            // Rust name, but that's ok for now as the only cases in the Schema end up with the same name presently.
                            for p in &property.incompatible_with {
                                if previous {
                                    condition += " && ";
                                }
                                previous = true;
                                condition += p;
                                condition += ".is_none()";
                            }

                            // The same but leave off the comma
                            write!(
                                output,
                                "            {}: if {} {{{}{}}} else {{ None }},\n",
                                property.name,
                                condition,
                                value,
                                if cloned { "" } else { ".clone()" }
                            )
                            .unwrap();
                        } else {
                            // The same but leave off the comma
                            write!(output, "            {}: {},\n", property.name, value).unwrap();
                        }
                    }
                    write!(output, "        }})\n").unwrap();

                    write!(output, "    }}\n").unwrap();

                    write!(output, "}}\n\n").unwrap();
                }
                RustType::Enum(rust_enum) => {
                    write!(output, "/// {}\n", rust_enum.description).unwrap();
                    write!(
                        output,
                        "#[derive(Debug, Clone)]pub enum {} {{\n",
                        rust_enum.name
                    )
                    .unwrap();
                    for member in rust_enum.members.iter() {
                        if let Some(description) = &member.description {
                            write!(output, "    /// {}\n", description).unwrap();
                        }

                        if let Some(value) = member.value {
                            write!(output, "    {} = {:?},\n", member.name, value).unwrap();
                        } else {
                            write!(output, "    {},\n", member.name,).unwrap();
                        }
                    }
                    write!(output, "}}\n\n").unwrap();

                    // Implement serialization for this enum
                    write!(
                        output,
                        "impl<S: Serializer> Serialize<S> for {} {{\n",
                        rust_enum.name
                    )
                    .unwrap();
                    write!(output, "    fn serialize(&self, serializer: &mut S) {{\n").unwrap();
                    write!(output, "        match self {{\n").unwrap();
                    for member in rust_enum.members.iter() {
                        match &member.json_value {
                            JsonEnumValue::Integer(i) => {
                                write!(
                                    output,
                                    "            Self::{} => {:?}.serialize(serializer),\n",
                                    member.name, i
                                )
                                .unwrap();
                            }
                            JsonEnumValue::String(s) => {
                                write!(
                                    output,
                                    "            Self::{} => {:?}.serialize(serializer),\n",
                                    member.name, s
                                )
                                .unwrap();
                            }
                        }
                    }
                    write!(output, "        }}\n").unwrap();

                    write!(output, "    }}\n").unwrap();
                    write!(output, "}}\n").unwrap();

                    // Implement deserialization
                    write!(
                        output,
                        "impl<'a, D: Deserializer<'a>> Deserialize<'a, D> for {} {{\n",
                        rust_enum.name
                    )
                    .unwrap();
                    {
                        write!(
                            output,
                            "    fn deserialize(deserializer: &mut D) -> Option<Self> {{\n"
                        )
                        .unwrap();

                        let enum_type = match &rust_enum.members[0].json_value {
                            JsonEnumValue::String(_) => {
                                write!(output, "        let value = deserializer.string()?;\n")
                                    .unwrap();
                                write!(output, "        Some(match &*value {{\n").unwrap();

                                0
                            }
                            JsonEnumValue::Integer(_) => {
                                write!(output, "        let value = deserializer.i64()?;\n")
                                    .unwrap();
                                write!(output, "        Some(match value {{\n").unwrap();

                                1
                            }
                        };
                        // Need to deserialize based on enum type.
                        for member in rust_enum.members.iter() {
                            match enum_type {
                                0 => {
                                    if let JsonEnumValue::String(s) = &member.json_value {
                                        write!(
                                            output,
                                            "            \"{}\" => Self::{},\n",
                                            s, member.name
                                        )
                                        .unwrap();
                                    }
                                }
                                1 => {
                                    if let JsonEnumValue::Integer(s) = &member.json_value {
                                        write!(
                                            output,
                                            "            {:?} => Self::{},\n",
                                            s, member.name
                                        )
                                        .unwrap();
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        write!(output, "        _ => None?\n").unwrap();
                        write!(output, "        }})\n").unwrap();
                        write!(output, "    }}\n").unwrap();
                    }
                    write!(output, "}}\n\n").unwrap();
                }
                _ => unimplemented!(),
            }
        }
        output
    }
}

fn main() {
    let source = std::fs::read_to_string("schema/glTF.schema.json").unwrap();
    let json = kserde::Thing::from_json(&source).expect("Could not parse JSON");
    let mut parser = Parser::new();
    let schema = parser.parse_schema(&json);

    let mut rust_generator = RustGenerator::new();
    let result = rust_generator.generate(&schema);
    std::fs::write("../src/gltf_json.rs", result).unwrap();
}
