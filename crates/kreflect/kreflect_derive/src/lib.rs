use kreflect_common::*;
use proc_macro::TokenStream;

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(item: TokenStream) -> TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    let mut parser = Parser::new(&rust_tokens);
    let _parse_result = parser.parse().expect("Could not parse");
    // println!("PARSE RESULT: {:#?}", _parse_result);
    // panic!();
    "".parse().unwrap()
}
/*
#[proc_macro_derive(Reflect)]
pub fn derive_reflect(item: TokenStream) -> TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    println!("TOKENS: {:#?}", rust_tokens);
    let mut parser = Parser::new(&rust_tokens);
    let parse_result = parser.parse().expect("Could not parse");

    //let stream = TokenStream::new();

    // panic!("REFLECT: {:#?}", parse_result);
    match &parse_result {
        Value::Struct(s) => {
            let mut reflection_output = String::new();
            reflection_output += "kreflect::Value::Struct (kreflect::Struct {\n";
            reflection_output += "  name: \"";
            reflection_output += &s.name;
            reflection_output += "\",\n";
            reflection_output += "fields: Fields::";
            match &s.fields {
                Fields::Tuple(_) => {
                    todo!("TUPLE DERIVE NOT YET IMPLEMENTED");
                }
                Fields::Struct(v) => {
                    println!("STRUCT: {:#?}", v);
                    reflection_output += "Struct(vec![";

                    for member in v {
                        let visibility_string = match member.visibility {
                            Visibility::Private => "Visibility::Private",
                            Visibility::Pub => "Visibility::Pub",
                            Visibility::Crate => "Visibility::Crate",
                            Visibility::Super => "Visibility::Super",
                        };

                        reflection_output += &format!(
                            r#"Field {{
                            name: Some("{}"),
                            _type: todo!(),
                            visibility: {}
                        }},"#,
                            member.name.as_ref().unwrap(),
                            //   member._type,
                            visibility_string
                        );
                    }
                    reflection_output += "]),";
                }
                Fields::Unit => reflection_output += "Unit,",
            }
            reflection_output += "generic_lifetimes: vec![";
            for lifetime in &s.generic_lifetimes {
                reflection_output += "\"";
                reflection_output += lifetime;
                reflection_output += "\"";
            }
            reflection_output += "],";

            reflection_output += "generic_types: vec![";
            for _type in &s.generic_types {
                reflection_output += "\"";
                reflection_output += _type;
                reflection_output += "\"";
            }
            reflection_output += "],";
            reflection_output += "visibility: Visibility::";
            reflection_output += match s.visibility {
                Visibility::Crate => "Crate",
                Visibility::Private => "Private",
                Visibility::Pub => "Pub",
                Visibility::Super => "Super",
            };
            reflection_output += ",";
            reflection_output += "})";

            println!("REFLECTION OUTPUT: {}", reflection_output);

            let mut lifetimes_and_generics = String::new();
            for lifetime in &s.generic_lifetimes {
                lifetimes_and_generics.push('\'');
                lifetimes_and_generics += lifetime;
                lifetimes_and_generics.push(',');
            }
            for generic in &s.generic_types {
                lifetimes_and_generics += generic;
                lifetimes_and_generics.push(',');
            }
            // panic!("reflection output: {}", reflection_output);
            let implementation = format!(
                r#"
            impl<{}> Reflect for {}<{}> {{
                fn type_name() -> &'static str {{
                    &"{}"
                }}

                fn reflect() -> Value<'static> {{
                    {}
                }}
            }}
            "#,
                lifetimes_and_generics, s.name, lifetimes_and_generics, s.name, reflection_output
            );

            return implementation.parse().unwrap();
        }
        _ => panic!("Non-structs are not yet supported"),
    }

    // stream
}
*/
