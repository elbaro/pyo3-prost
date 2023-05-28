mod object;
mod parse;

use quote::ToTokens;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn pyclass_for_prost_struct(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = pyclass_for_prost_struct_impl(input);
    proc_macro::TokenStream::from(output)
}

fn pyclass_for_prost_struct_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let Ok(struct_) = syn::parse2::<syn::ItemStruct>(input.clone()) else {
        return input;
    };

    let fields = parse::parse_fields(&struct_.fields);
    let proxy_types = object::derive_object(&struct_.ident, &fields);

    struct_
        .into_token_stream()
        .into_iter()
        .chain(proxy_types.into_iter())
        // .chain(field_struct.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use std::str::FromStr;
        let ts = proc_macro2::TokenStream::from_str(
            "#[derive(Clone, PartialEq, ::prost::Message)]\npub struct MarginUpdate { a: i32, pub b:String,}",
        )
        .unwrap();
        println!("{}", super::pyclass_for_prost_struct_impl(ts));
    }
}

// struct RustAttribute<Root, Field>(OwningRef<Arc<Root>, Field>);
