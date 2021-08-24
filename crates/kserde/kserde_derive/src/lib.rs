use kreflect_common::*;

#[proc_macro_derive(SerializeDeserialize, attributes(skip))]
pub fn derive_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut rust_tokens = Vec::new();
    token_stream_to_rust_tokens(item, &mut rust_tokens);

    println!("TOKENS: {:#?}", rust_tokens);
    let mut parser = Parser::new(&rust_tokens);
    let parse_result = parser.parse().expect("Could not parse");
    let mut output_string = String::new();
    output_string += &kserde_derive_impl::kserde_serialize_impl(&parse_result);
    output_string += &kserde_derive_impl::kserde_deserialize_impl(&parse_result);
    println!("OUTPUT STRING: {}", output_string);
    output_string.parse().unwrap()
}
