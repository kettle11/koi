use std::borrow::Cow;

use kreflect_common::*;

fn serialize_fields(properties: &mut String, fields: &[Field]) {
    for (i, field) in fields.iter().enumerate() {
        let skip = field_contains_attribute(field, "skip");
        if !skip {
            if let Some(name) = field.name.as_ref() {
                *properties += &format!(
                    "    serializer.property(\"{}\");\n    serializer.value(&self.{});\n",
                    name, name
                );
            } else {
                *properties += &format!(
                    "    serializer.property(\"{}\");\n    serializer.value(&self.{});\n",
                    i, i
                );
            }
        }
    }
}

fn serialize_enum_fields(properties: &mut String, fields: &[Field]) {
    for (i, field) in fields.iter().enumerate() {
        let skip = field_contains_attribute(field, "skip");
        if !skip {
            if let Some(name) = field.name.as_ref() {
                *properties += &format!(
                    "    serializer.property(\"{}\");\n    serializer.value(&{});\n",
                    name, name
                );
            } else {
                *properties += &format!(
                    "    serializer.property(\"{}\");\n    serializer.value(&i{});\n",
                    i, i
                );
            }
        }
    }
}

pub fn kserde_serialize_impl(value: &Value) -> String {
    let generic_lifetimes;
    let mut generic_types;
    let generic_consts;
    let name;
    let generic_args;

    let mut where_clause = String::new();
    let mut properties;

    match value {
        Value::Struct(_struct) => {
            let (generic_lifetimes0, generic_types0, generic_consts0) =
                _struct.generic_parameters.lifetimes_types_consts();
            generic_lifetimes = generic_lifetimes0;
            generic_types = generic_types0;
            generic_consts = generic_consts0;

            generic_types += "KSer: kserde::Serializer, ";
            name = &_struct.name;
            generic_args = _struct.generic_parameters.as_args();

            if !_struct.generic_parameters.0.is_empty() {
                where_clause += "where ";
                for generic_arg in _struct.generic_parameters.0.iter() {
                    where_clause += &format!("{}: Serialize<KSer>, \n", generic_arg.as_string());
                }
            }

            properties = String::new();
            match &_struct.fields {
                Fields::Struct(fields) => {
                    serialize_fields(&mut properties, fields);
                }
                Fields::Tuple(fields) => {
                    serialize_fields(&mut properties, fields);
                }
                Fields::Unit => {}
            }
        }
        Value::Enum(_enum) => {
            let (generic_lifetimes0, generic_types0, generic_consts0) =
                _enum.generic_parameters.lifetimes_types_consts();
            generic_lifetimes = generic_lifetimes0;
            generic_types = generic_types0;
            generic_consts = generic_consts0;

            generic_types += "KSer: kserde::Serializer, ";
            name = &_enum.name;
            generic_args = _enum.generic_parameters.as_args();

            if !_enum.generic_parameters.0.is_empty() {
                where_clause += "where ";
                for generic_arg in _enum.generic_parameters.0.iter() {
                    where_clause += &format!("{}: Serialize<KSer>, \n", generic_arg.as_string());
                }
            }

            let mut variants = String::new();
            for variant in &_enum.variants {
                let mut variant_match = format!("    {}::{}", _enum.name, variant.name);
                match &variant.fields {
                    Fields::Tuple(fields) => {
                        variant_match.push('(');
                        for (i, field) in fields.iter().enumerate() {
                            if !field_contains_attribute(field, "skip") {
                                variant_match.push('i');
                                variant_match += &i.to_string();
                                variant_match.push(',')
                            }
                        }
                        // Serialize the variant's name
                        variant_match += &format!(
                            ") => {{\n    serializer.property(\"{}\");\n    serializer.begin_object();\n",
                            variant.name
                        );

                        serialize_enum_fields(&mut variant_match, fields);
                        variant_match += "serializer.end_object();\n},";
                    }
                    Fields::Struct(fields) => {
                        variant_match.push('{');
                        for field in fields.iter() {
                            if !field_contains_attribute(field, "skip") {
                                variant_match += field.name.as_ref().unwrap();
                                variant_match.push(',')
                            }
                        }
                        variant_match += &format!(
                            "}} => {{\n    serializer.property(\"{}\");\n    serializer.begin_object();\n",
                            variant.name
                        );

                        serialize_enum_fields(&mut variant_match, fields);

                        variant_match += "serializer.end_object();\n},";
                    }
                    Fields::Unit => {
                        variant_match += &format!(
                            " => {{\n    serializer.property(\"{}\");\n    serializer.begin_object();\n    serializer.end_object();\n}},\n",
                            variant.name
                        )
                    }
                }
                variants += &variant_match;

                // Need to emit a name here.
                //variant.fields
                // Left off here, need to generate serialization code for each variant.
            }
            properties = format!(
                r#"match self {{
                    {}
                }}"#,
                &variants
            );
        }
        _ => {
            panic!("Can't serialize functions")
        }
    }

    format!(
        r#"impl<{}{}{}> kserde::Serialize<KSer> for {}{} {} {{
        fn serialize(&self, serializer: &mut KSer) {{
        serializer.begin_object();
        {}
        serializer.end_object();
    }}
}}"#,
        generic_lifetimes,
        generic_types,
        &generic_consts,
        name,
        generic_args,
        where_clause,
        properties
    )
}

pub fn deserialize_fields(
    fields: &Fields,
    properties_declaration: &mut String,
    deserialize_match: &mut String,
    property_assignment: &mut String,
) {
    let fields = match fields {
        Fields::Struct(f) => f,
        Fields::Tuple(f) => f,
        Fields::Unit => return,
    };
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
    let mut generic_lifetimes;
    let mut generic_types;
    let name;
    let body;

    let generic_parameters;
    let mut where_clause = String::new();

    match value {
        Value::Struct(_struct) => {
            generic_parameters = &_struct.generic_parameters;
            name = &_struct.name;

            let mut deserialize_match = String::new();
            let mut properties_declaration = String::new();
            let mut property_assignment = String::new();

            if !_struct.generic_parameters.0.is_empty() {
                where_clause += "where ";
                for generic_arg in _struct.generic_parameters.0.iter() {
                    where_clause += &format!(
                        "{}: Deserialize<'kserde, KDes>, \n",
                        generic_arg.as_string()
                    );
                }
            }

            deserialize_fields(
                &_struct.fields,
                &mut properties_declaration,
                &mut deserialize_match,
                &mut property_assignment,
            );
            body = format!(
                r#"
            {properties_declaration}
            while let Some(p) = deserializer.has_property() {{
                match &*p {{
                    {deserialize_match}
                    _ => {{}}
                }}
            }}
            Some(Self {{
                {property_assignment}
            }})
            "#
            )
        }
        Value::Enum(enum_) => {
            generic_parameters = &enum_.generic_parameters;
            name = &enum_.name;

            let mut enum_body_inner = String::new();

            for variant in &enum_.variants {
                let mut deserialize_match = String::new();
                let mut properties_declaration = String::new();
                let mut property_assignment = String::new();

                if !enum_.generic_parameters.0.is_empty() {
                    where_clause += "where ";
                    for generic_arg in enum_.generic_parameters.0.iter() {
                        where_clause += &format!(
                            "{}: Deserialize<'kserde, KDes>, \n",
                            generic_arg.as_string()
                        );
                    }
                }

                deserialize_fields(
                    &variant.fields,
                    &mut properties_declaration,
                    &mut deserialize_match,
                    &mut property_assignment,
                );

                let variant_name = &variant.name;
                match &variant.fields {
                    Fields::Struct(_) => {
                        enum_body_inner += &format!(
                            r#"
                            "{variant_name}" => {{
                                {properties_declaration}
                                while let Some(p) = deserializer.has_property() {{
                                    match &*p {{
                                        {deserialize_match}
                                        _ => {{}}
                                    }}
                                }}
                                {name}::{variant_name}{{{property_assignment}}}
                            }}
                        "#
                        );
                    }
                    Fields::Tuple(f) => {
                        let mut property_assignment = String::new();
                        for (i, field) in f.iter().enumerate() {
                            if !field_contains_attribute(field, "skip") {
                                property_assignment += &format!("f_{}?,", i);
                            }
                        }
                        enum_body_inner += &format!(
                            r#"
                            "{variant_name}" => {{
                                {properties_declaration}
                                while let Some(p) = deserializer.has_property() {{
                                    match &*p {{
                                        {deserialize_match}
                                        _ => {{}}
                                    }}
                                }}
                                {name}::{variant_name}({property_assignment})
                            }}
                        "#
                        );
                    }
                    Fields::Unit => {
                        enum_body_inner += &format!("\"{variant_name}\" => {name}::{variant_name},")
                    }
                }
            }

            body = format!(
                r#"
                let enum_name = deserializer.has_property()?;
                deserializer.begin_object().then(|| {{}})?;
                deserializer.end_object();
                
                Some(match &*enum_name {{
                    {enum_body_inner}
                    _ => None?
                }})
            "#
            );
        }
        _ => {
            panic!("Can't deserialize functions")
        }
    }

    let (generic_lifetimes0, generic_types0, generic_consts) =
        generic_parameters.lifetimes_types_consts();

    generic_lifetimes = generic_lifetimes0;
    generic_types = generic_types0;

    generic_lifetimes += "'kserde, ";
    generic_types += "KDes: kserde::Deserializer<'kserde>, ";

    let generic_args = generic_parameters.as_args();

    format!(
        r#"impl<{generic_lifetimes}{generic_types}{generic_consts}> kserde::Deserialize<'kserde, KDes> for {name}{generic_args} {where_clause} {{
    fn deserialize(deserializer: &mut KDes) -> Option<Self> {{
        deserializer.begin_object().then(|| {{}})?;
       let result = {{{body}}};
       deserializer.end_object();
       result
    }}
}}"#
    )
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
