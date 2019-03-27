extern crate proc_macro;


use
{
	quote      :: { quote                          } ,
	proc_macro :: { TokenStream                    } ,
	syn        :: { parse_macro_input, DeriveInput } ,
};


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

