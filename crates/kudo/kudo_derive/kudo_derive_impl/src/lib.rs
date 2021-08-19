use kreflect_common::*;

pub fn kudo_component_impl(value: &Value) -> String {
    let (name, generic_parameters) = match value {
        Value::Struct(s) => (&s.name, &s.generic_parameters),
        Value::Enum(e) => (&e.name, &e.generic_parameters),
    };

    format!(
        r#"
        impl{} ComponentTrait for {}{} {{
            fn clone_components(
                _entity_migrator: &mut EntityMigrator,
                items: &[Self],
            ) -> Option<Vec<Self>> {{
                Some(items.into())
            }}
        }}
    "#,
        &generic_parameters.as_impl_args(),
        &name,
        &generic_parameters.as_args(),
    )
}

pub fn kudo_non_clone_component_impl(value: &Value) -> String {
    let (name, generic_parameters) = match value {
        Value::Struct(s) => (&s.name, &s.generic_parameters),
        Value::Enum(e) => (&e.name, &e.generic_parameters),
    };

    format!(
        r#"
        impl{} ComponentTrait for {}{} {{
            fn clone_components(
                _entity_migrator: &mut EntityMigrator,
                _items: &[Self],
            ) -> Option<Vec<Self>> {{
               None
            }}
        }}
    "#,
        &generic_parameters.as_impl_args(),
        &name,
        &generic_parameters.as_args(),
    )
}
