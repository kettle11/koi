use kreflect_common::*;

#[proc_macro_derive(Component, attributes(skip))]
pub fn derive_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    // println!("TOKENS: {:#?}", rust_tokens);
    let mut parser = Parser::new(&rust_tokens);
    let parse_result = parser.parse().expect("Could not parse");
    let output_string = kecs_derive_impl::kecs_component_impl(&parse_result);
    // output_string += &kserde_derive_impl::kserde_deserialize_impl(&parse_result);
    // output_string += &kserde_derive_impl::kserde_serialize_impl(&parse_result);
    //println!("OUTPUT STRING: {}", output_string);

    output_string.parse().unwrap()
}

#[proc_macro_derive(ManualSerdeComponent, attributes(skip))]
pub fn manual_serde_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    let mut parser = Parser::new(&rust_tokens);
    let parse_result = parser.parse().expect("Could not parse");
    let output_string = kecs_derive_impl::kecs_component_impl(&parse_result);
    output_string.parse().unwrap()
}

#[proc_macro_derive(NotCloneComponent)]
pub fn derive_non_clone_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    // println!("TOKENS: {:#?}", rust_tokens);
    let mut parser = Parser::new(&rust_tokens);
    let parse_result = parser.parse().expect("Could not parse");
    let output_string = kecs_derive_impl::kecs_non_clone_component_impl(&parse_result);

    // println!("OUTPUT STRING: {}", output_string);

    output_string.parse().unwrap()
}
