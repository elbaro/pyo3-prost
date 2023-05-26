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

    // struct_
    //     .attrs
    //     .push(syn::parse_quote! {#[::pyo3::prelude::pyclass]});

    let fields = parse::parse_fields(&struct_.fields);
    let proxy_types = object::derive_object(&struct_.ident, &fields);

    // optional -> Option<T>
    // repeated -> Vec<T>

    // match (field.cardinality, type_kind) {
    //     (_, FieldType::Message) => {
    //         // MyType => MyTypeField
    //         // Vec<MyType> => Vec<MyTypeField>
    //         // Option<MyType> => Option<MyTypeField>

    //         match field.cardinality {
    //             Cardinality::Unknown | Cardinality::Optional => {
    //                 let inner_type = extract_inner_type(&ty);
    //                 let proxy_type = syn::Ident::new(
    //                     &format!("{}Field", inner_type.to_token_stream()),
    //                     ident.span(),
    //                 );

    //                 quote::quote! {
    //                     #[getter]
    //                     pub fn #ident(&self) -> Option<#proxy_type> {
    //                         self.0.as_ref().map(|field| {
    //                             #proxy_type {
    //                                 owner: owner.clone(),
    //                                 field: field,
    //                             }
    //                         })

    //                     }
    //                 }
    //             }
    //             Cardinality::Required => {
    //                 let proxy_type = syn::Ident::new(
    //                     &format!("{}Field", ty.to_token_stream()),
    //                     ident.span(),
    //                 );

    //                 quote::quote! {
    //                     #[getter]
    //                     pub fn #ident(&self) -> &#proxy_type {
    //                         &self.0.#ident
    //                     }
    //                 }
    //             }
    //             Cardinality::Repeated => {
    //                 let inner_type = extract_inner_type(&ty);
    //                 let proxy_type = syn::Ident::new(
    //                     &format!("{}Field", inner_type.to_token_stream()),
    //                     ident.span(),
    //                 );

    //                 quote::quote! {
    //                     #[getter]
    //                     pub fn #ident(&self) -> Vec<#proxy_type> {
    //                         #proxy_type {
    //                             owner: self.0.clone(),
    //                             field: &self.0.#ident,
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         // panic!();
    //     }
    //     (_, FieldType::String) => {
    //         quote::quote!("unknown cardinality")
    //     }
    //     (Cardinality::Optional, _) => {
    //         quote::quote!("unknown cardinality")
    //     }
    //     (Cardinality::Repeated, _) => {
    //         // Vec<i8>
    //         quote::quote! {
    //             #[getter]
    //             pub fn #ident(&self) -> #ty {
    //                 &self.0.#ident
    //             }
    //         }
    //     }
    //     (Cardinality::Required, _) => {
    //         quote::quote! {
    //             #[getter]
    //             pub fn #ident(&self) -> #ty {
    //                 &self.0.#ident
    //             }
    //         }
    //     }
    //     (Cardinality::Unknown, _) => {
    //         quote::quote!("unknown cardinality")
    //     }
    // }

    // let field_struct = quote::quote! {
    //     struct asdf {
    //         owner: Arc<#struct_name>,
    //         field: &#struct_name,
    //     }
    //     #[pymethods]
    //     impl asdf {
    //         #(
    //             #field_tokens
    //         )*
    //     }
    // };

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
