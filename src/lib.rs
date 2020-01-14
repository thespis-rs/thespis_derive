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
	quote      :: { quote                          } ,
	proc_macro :: { TokenStream                    } ,
	syn        :: { parse_macro_input, DeriveInput } ,
};


/// Generate a basic impl of Actor without any methods.
//
#[ proc_macro_derive( Actor ) ]
//
pub fn derive_actor( input: TokenStream ) -> TokenStream
{
	let input = parse_macro_input!( input as DeriveInput );

	let ( impl_generics, ty_generics, where_clause ) = input.generics.split_for_impl();

	let name = input.ident;

	// The generated impl.
	//
	let expanded = quote!
	{
		impl #impl_generics thespis::Actor for #name #ty_generics #where_clause
		{}
	};

	proc_macro::TokenStream::from( expanded )
}

