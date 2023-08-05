#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

#![ doc    ( html_root_url = "https://docs.rs/thespis_derive" ) ]
#![ deny   ( missing_docs                                     ) ]
#![ forbid ( unsafe_code                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting               ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
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
///
/// For usage, please look at tests and examples in the _thespis_impl_ crate.
/// The [guide](https://thespis-rs.github.io/thespis_guide/thespis_impl/thespis_impl.html) shows
/// what this desugars to.
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
		fn #name( #args ) -> ::thespis::Return< '_, #ret > { ::std::boxed::Box::pin
		(
			async move #body
		)}
	};

	tokens.into()
}


/// Implement an async trait method for thespis traits.
///
/// For usage, please look at tests and examples in the _thespis_impl_ crate.
/// The [guide](https://thespis-rs.github.io/thespis_guide/thespis_impl/thespis_impl.html) shows
/// what this desugars to.
//
#[ proc_macro_attribute ]
//
pub fn async_fn_local( _args: TokenStream, item: TokenStream ) -> TokenStream
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
		fn #name( #args ) -> ::thespis::ReturnNoSend< '_, #ret > { ::std::boxed::Box::pin
		(
			async move #body
		)}
	};

	tokens.into()
}


/// Implement an async trait method for thespis traits. Like async_fn_local, but also
/// creates a default `handle` function that will panic if called.
///
/// For usage, please look at tests and examples in the _thespis_impl_ crate.
/// The [guide](https://thespis-rs.github.io/thespis_guide/thespis_impl/thespis_impl.html) shows
/// what this desugars to.
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
		fn #name( #args ) -> ::thespis::ReturnNoSend< '_, #ret > { ::std::boxed::Box::pin
		(
			async move #body
		)}

		#[allow(unused_variables)]
		fn handle( #args ) -> ::thespis::Return< '_, #ret >
		{
			unreachable!( "Non-Send actor cannot be spawned on a threadpool. Use spawn_local." );
		}
	};

	tokens.into()
}

