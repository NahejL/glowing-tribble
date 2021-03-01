extern crate proc_macro;

use quote;
use syn;

// #[proc_macro_derive(SomeMacro)]
// pub fn some_macro_derive( input: proc_macro::TokenStream ) -> proc_macro::TokenStream {

//   let ast: &syn::DeriveInput = match syn::parse( input ) {
//     Ok( ast ) => ast,
//     Err( error ) => panic!( "Failed to syn parse input with :{}", error )
//   };

//   let name = &ast.ident;
//   let gen = quote! {
//     impl SomeMacro for #name {
//       fn some_fn() {
//         println! ( "macro code" )
//       }
//     }
//   }

//   gen.into()

// }

// #[proc_macro_attribute] 
// pub fn some_fn( attribute: proc_macro::TokenStream, item: proc_macro::TokenStream ) -> proc_macro::TokenStream {

// }

// #[proc_macro]
// pub fn some_fn( input: proc_macro::TokenStream ) -> proc_macro::TokenStream {

// }