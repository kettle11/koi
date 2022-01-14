use std::borrow::Cow;

use kreflect_common::*;

fn serialize_fields(properties: &mut String, fields: &Vec<Field>) {
    for (i, field) in fields.iter().enumerate() {
        let skip = field_contains_attribute(field, "skip");
        if !skip {
            if let Some(name) = field.name.as_ref() {
                *properties += &format!("    serializer.property(\"{}\", &self.{});\n", name, name);
            } else {
                *properties += &format!("    serializer.property(\"{}\", &self.{});\n", i, i);
            }
        }
    }
}

pub fn kserde_serialize_impl(value: &Value) -> String {
    match value {
        Value::Struct(_struct) => {
            let (generic_lifetimes, mut generic_types, generic_consts) =
                _struct.generic_parameters.lifetimes_types_consts();

            generic_types += "KSer: kserde::Serializer, ";

            let generic_args = _struct.generic_parameters.as_args();

            let mut properties = String::new();
            match &_struct.fields {
                Fields::Struct(fields) => {
                    serialize_fields(&mut properties, fields);
                }
                Fields::Tuple(fields) => {
                    serialize_fields(&mut properties, fields);
                }
                Fields::Unit => todo!(),
            }

            format!(
                r#"impl<{}{}{}> kserde::Serialize<KSer> for {}<{}> {{
    fn serialize(&self, serializer: &mut KSer) {{
        serializer.begin_object();
{}
        serializer.end_object();
    }}
}}"#,
                generic_lifetimes,
                generic_types,
                &generic_consts,
                _struct.name,
                generic_args,
                properties
            )
        }
        Value::Enum(_) => {
            todo!()
        }
    }
}

pub fn deserialize_fields(
    properties_declaration: &mut String,
    deserialize_match: &mut String,
    property_assignment: &mut String,
    fields: &Vec<Field>,
) {
    for (i, field) in fields.iter().enumerate() {
        let name: Cow<str> = if let Some(field_name) = field.name.as_ref() {
            field_name.clone()
        } else {
            i.to_string().into()
        };
        let _type = field._type.as_string();

        let skip = field_contains_attribute(field, "skip");

        if !skip {
            if _type.get(0..6).map_or(false, |s| s == "Option") {
                *properties_declaration += &format!("    let mut f_{}: {} = None;\n", name, _type);
                *deserialize_match += &format!(
                    "                \"{}\" => f_{} = Some(<{}>::deserialize(deserializer)?),\n",
                    name,
                    name,
                    &_type[7.._type.len() - 1]
                );
                *property_assignment += &format!("    {}: f_{},\n", name, name);
            } else {
                *properties_declaration +=
                    &format!("    let mut f_{}: Option<{}> = None;\n", name, _type);
                *deserialize_match += &format!(
                    "                \"{}\" => f_{} = Some(<{}>::deserialize(deserializer)?),\n",
                    name, name, _type
                );
                *property_assignment += &format!("        {}: f_{}?,\n", name, name);
            }
        } else {
            // Assign a default value to the property if it's skipped.
            *property_assignment +=
                &format!("        {}: std::default::Default::default(),\n", name);
        }
    }
}

pub fn kserde_deserialize_impl(value: &Value) -> String {
    match value {
        Value::Struct(_struct) => {
            let (mut generic_lifetimes, mut generic_types, generic_consts) =
                _struct.generic_parameters.lifetimes_types_consts();

            generic_lifetimes += "'kserde, ";
            generic_types += "KDes: kserde::Deserializer<'kserde>, ";

            let generic_args = _struct.generic_parameters.as_args();

            let mut deserialize_match = String::new();
            let mut properties_declaration = String::new();
            let mut property_assignment = String::new();

            match &_struct.fields {
                Fields::Struct(fields) => deserialize_fields(
                    &mut properties_declaration,
                    &mut deserialize_match,
                    &mut property_assignment,
                    fields,
                ),
                Fields::Tuple(fields) => deserialize_fields(
                    &mut properties_declaration,
                    &mut deserialize_match,
                    &mut property_assignment,
                    fields,
                ),
                Fields::Unit => todo!(),
            }
            format!(
                r#"impl<{}{}{}> kserde::Deserialize<'kserde, KDes> for {}<{}> {{
    fn deserialize(deserializer: &mut KDes) -> Option<Self> {{
        deserializer.begin_object().then(|| {{}})?;
{}
        while let Some(p) = deserializer.has_property() {{
            match &*p {{
{}              _ => {{}}
            }}
        }}
        Some(Self {{
{}
        }})
    }}
}}"#,
                generic_lifetimes,
                generic_types,
                &generic_consts,
                _struct.name,
                generic_args,
                properties_declaration,
                deserialize_match,
                property_assignment
            )
        }
        Value::Enum(_) => {
            todo!()
        }
    }
}

/// Check if a [Field] contains an attribute name.
fn field_contains_attribute(field: &Field, attribute: &str) -> bool {
    field.attributes.iter().any(|a| {
        a.path.segments.iter().any(|s| match &s.path_segment_type {
            PathSegmentType::Named(a) => attribute == a,
            _ => false,
        })
    })
}

#[test]
fn kersde_impl() {
    let value = Value::Struct(Struct {
        name: "Thing".into(),
        visibility: Visibility::Private,
        generic_parameters: GenericParams(Vec::new()),
        fields: Fields::Struct(vec![Field {
            name: Some("x".into()),
            _type: Type::Name(Path::new(&["f32".into()])),
            visibility: Visibility::Pub,
            attributes: Vec::new(),
        }]),
    });

    println!("{}", kserde_deserialize_impl(&value));
}
