use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro]
pub fn identity(tokens: TokenStream) -> TokenStream {
	tokens
}

#[proc_macro]
pub fn expr_identity(tokens: TokenStream) -> TokenStream {
	let original = tokens.clone();
	let expr: syn::Expr = syn::parse(tokens).unwrap();
	let output = expr.to_token_stream().into();

	assert_eq!(format!("{:#?}", original), format!("{:#?}", output));
	output
}
