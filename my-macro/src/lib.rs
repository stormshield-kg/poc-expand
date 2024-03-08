#[proc_macro_attribute]
pub fn attr_macro(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let token_stream = quote:: quote!(
        #input
        mod tmp {
            #[derive(Clone)]
            #input
        }
    );

    eprintln!("\n\n{}\n\n", token_stream);

    token_stream.into()
}
