use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn slash_command(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
