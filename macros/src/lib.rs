extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::Ident;

#[proc_macro_attribute]
pub fn os_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    if &input.sig.inputs.len() > &0 {
        panic!("OS test can't have arguments.")
    }

    let fn_ident= &input.sig.ident;
    let fn_name = fn_ident.to_string();
    let const_ident = Ident::new(&format!("{}_CONST", fn_name), Span::call_site());

    let tokens = quote! {
        #[cfg(test)]
        #[test_case]
        const #const_ident: crate::os_test::OSTest = crate::os_test::OSTest { name: #fn_name, test: &#fn_ident };
        #[cfg(test)]
        #input
    };

    TokenStream::from(tokens)
}
