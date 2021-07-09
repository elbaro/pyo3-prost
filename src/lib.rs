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
    if let Ok(mut struct_) = syn::parse2::<syn::ItemStruct>(input.clone()) {
        struct_
            .attrs
            .push(syn::parse_quote! {#[::pyo3::prelude::pyclass]});
        if let syn::Fields::Named(fields_named) = &mut struct_.fields {
            for field in fields_named.named.iter_mut() {
                // exception: we do not support `oneof` fields yet
                let is_oneof = field.attrs.iter().any(|attr| {
                    if attr.path.is_ident("prost") {
                        if let Ok(syn::Meta::List(list)) = attr.parse_meta() {
                            return list.nested.iter().any(|nested_meta| {
                                if let syn::NestedMeta::Meta(meta) = nested_meta {
                                    if let syn::Meta::NameValue(nv) = meta {
                                        if nv.path.is_ident("oneof") {
                                            return true;
                                        }
                                    }
                                }
                                false
                            });
                        }
                    }
                    false
                });

                if !is_oneof {
                    field.attrs.push(syn::parse_quote! {
                        #[pyo3(get, set)]
                    });
                }
            }
        }

        let struct_name = &struct_.ident;
        let impl_ = quote::quote! {
            #[::pyo3::prelude::pymethods]
            impl #struct_name {
                #[new]
                pub fn new() -> Self {
                    Self::default()
                }

                #[staticmethod]
                pub fn decode(bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<Self> {
                    let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                    <Self as ::prost::Message>::decode(bytes).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })
                }

                pub fn decode_merge(slf: &::pyo3::pycell::PyCell<#struct_name>, py: ::pyo3::Python, bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<()> {
                    let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                    {
                        let mut obj_mut = slf.borrow_mut();
                        <Self as ::prost::Message>::merge(::core::ops::DerefMut::deref_mut(&mut obj_mut), bytes).map_err(|e| {
                            ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                        })?;
                    }
                    Ok(())
                }

                /// why slow?
                /// because it allocates the Rust buffer
                /// and then copy to Python buffer.
                pub fn encode_slow(&self, py: ::pyo3::Python) -> ::pyo3::PyResult<::pyo3::PyObject> {
                    let mut buf = Vec::<u8>::new();
                    ::prost::Message::encode(self, &mut buf).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                    Ok(pyo3::types::PyBytes::new(py, &buf).into())
                }

                pub fn clear(&mut self) {
                    *self = Default::default();
                }
            }

            #[::pyo3::prelude::pyproto]
            impl ::pyo3::class::basic::PyObjectProtocol for #struct_name {
                fn __repr__(&self) -> ::pyo3::PyResult<String> {
                    Ok(format!("{:?}", self))
                }
                fn __str__(&self) -> ::pyo3::PyResult<String> {
                    Ok(format!("{:#?}", self))
                }
            }
        };

        struct_
            .into_token_stream()
            .into_iter()
            .chain(impl_.into_iter())
            .collect()
    } else {
        input
    }
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
