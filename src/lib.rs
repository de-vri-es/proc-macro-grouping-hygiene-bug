use proc_macro::TokenStream;

#[proc_macro]
pub fn identity(tokens: TokenStream) -> TokenStream {
	tokens.clone()
}

#[proc_macro]
pub fn manual_identity(tokens: TokenStream) -> TokenStream {
	let mut output = TokenStream::new();
	output.extend(tokens);
	output
}
