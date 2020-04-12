// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//!
#![ cfg_attr( feature = "external_doc", feature(external_doc)         ) ]
#![ cfg_attr( feature = "external_doc", doc(include = "../README.md") ) ]
//
#![ doc    ( html_root_url = "https://docs.rs/thespis_derive" ) ]
#![ deny   ( missing_docs                                     ) ]
#![ forbid ( unsafe_code                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting               ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


use
{
	quote      :: { quote                                             } ,
	proc_macro :: { TokenStream                                       } ,
	syn        :: { parse_macro_input, parse, DeriveInput, ReturnType } ,
};


/// Generate a basic impl of Actor without any methods.
//
#[ proc_macro_derive( Actor ) ]
//
pub fn derive_actor( input: TokenStream ) -> TokenStream
{
	let input = parse_macro_input!( input as DeriveInput );
	let name  = input.ident;

	let ( impl_generics, ty_generics, where_clause ) = input.generics.split_for_impl();

	// The generated impl.
	//
	let expanded = quote!
	{
		impl #impl_generics thespis::Actor for #name #ty_generics #where_clause
		{}
	};

	TokenStream::from( expanded )
}


/// Implement an async trait method for thespis traits.
//
#[ proc_macro_attribute ]
//
pub fn async_fn( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input: syn::ItemFn = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};


	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let ret = match &input.sig.output
	{
		ReturnType::Default      => quote!( ()   ) ,
		ReturnType::Type(_, ret) => quote!( #ret ) ,
	};

	let tokens = quote!
	{
		#( #attrs )*
		//
		fn #name( #args ) -> thespis::Return< '_, #ret > { Box::pin
		(
			async move #body
		)}
	};

	tokens.into()
}


/// Implement an async trait method for thespis traits.
//
#[ proc_macro_attribute ]
//
pub fn async_fn_nosend( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input: syn::ItemFn = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};


	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let ret = match &input.sig.output
	{
		ReturnType::Default      => quote!( ()   ) ,
		ReturnType::Type(_, ret) => quote!( #ret ) ,
	};

	let tokens = quote!
	{
		#( #attrs )*
		//
		fn #name( #args ) -> thespis::ReturnNoSend< '_, #ret > { Box::pin
		(
			async move #body
		)}
	};

	tokens.into()
}

