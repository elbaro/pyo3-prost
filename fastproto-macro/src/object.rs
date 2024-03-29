// ## Object or Option<Object>

// ```rs
// #[pyclass]
// struct TweetOwned(Arc<Tweet>);
// #[pymethods]
// impl TweetRef{
//   #[getter]
//   fn text(self: &PyAny) -> &PyAny {
//     if self.0.text.is_none() { return None; }
//     let arc_self: Arc<Self> = self.0.clone();
//     TextRef(
//       owner: arc_self as Arc<dyn Send + Sync>,
//       borrowed: std::mem::transmute::<&'_ Text, &'static Text>( &arc_self.text ),
//     ) to pyobject
//   }
// }

// struct Tweet {
//   text: Text
// }

use prost_types::field::Cardinality;
use quote::format_ident;
use syn::Ident;

use crate::parse::{Field, FieldType};

pub fn derive_object(ident: &Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let ref_ident = format_ident!("{}Ref", ident);
    let impl_proto_api = quote::quote! {
        #[::pyo3::pymethods]
        impl #ref_ident {
            #[new]
            pub fn py_new() -> Self {
                Self::default()
            }

            #[staticmethod]
            pub fn decode(bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<Self> {
                let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                <#ident as ::prost::Message>::decode(bytes).map(|data| {
                    let owner = ::std::sync::Arc::new(data);
                    <Self as ::fastproto_lib::Ref<_>>::new_owned(owner)
                }).map_err(|e| {
                    ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                })
            }

            pub fn decode_merge(&mut self, py: ::pyo3::Python, bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<()> {
                let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                let borrowed = unsafe { ::std::mem::transmute::<& #ident, &mut #ident>(self.borrowed) };
                {
                    <#ident as ::prost::Message>::merge(borrowed, bytes).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                }
                Ok(())
            }

            pub fn encode<'a>(&self, py: ::pyo3::Python<'a>) -> ::pyo3::PyResult<&'a ::pyo3::types::PyBytes> {
                Ok(::pyo3::types::PyBytes::new_with(py, ::prost::Message::encoded_len(self.borrowed), |mut py_buf: &mut [u8]| {
                    ::prost::Message::encode(self.borrowed, &mut py_buf).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                    Ok(())
                })?)
            }

            pub fn clear(&mut self) {
                let borrowed = unsafe { ::std::mem::transmute::<& #ident, &mut #ident>(self.borrowed) };
                *borrowed = Default::default();
            }

            fn __repr__(&self) -> String {
                format!("{:?}", self.borrowed)
            }
            fn __str__(&self) -> String {
                format!("{:#?}", self.borrowed)
            }
        }
    };

    let getters = fields.iter().map(|field| {
        let field_ident = &field.ident;

        match &field.type_kind {
            FieldType::_Map => {
                quote::quote!{}
            }
            FieldType::Scalar => {
                let field_ty = &field.ty;

                match &field.cardinality {
                    Cardinality::Unknown => unreachable!(),
                    Cardinality::Optional | Cardinality::Required => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> #field_ty {
                                self.borrowed.#field_ident.clone()
                            }
                        }
                    }
                    Cardinality::Repeated => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> #field_ty {
                                // TODO: use proxy list
                                self.borrowed.#field_ident.clone()
                            }
                        }
                    }
                }
            }
            FieldType::Message => {
                let message_ident = field.message_ident.as_ref().unwrap();
                let ref_ident = format_ident!("{}Ref", message_ident);

                match field.cardinality {
                    Cardinality::Unknown => unreachable!(),
                    Cardinality::Required => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> #ref_ident {
                                let borrowed = unsafe { std::mem::transmute::<&'_ #message_ident, &'static #message_ident>( &self.borrowed.#field_ident ) };

                                #ref_ident {
                                    owner: self.owner.clone(),
                                    borrowed,
                                }

                            }
                        }
                    },
                    Cardinality::Optional => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> ::core::option::Option<#ref_ident> {
                                if self.borrowed.#field_ident.is_none() { return None; }
                                let borrowed = unsafe { std::mem::transmute::<&'_ #message_ident, &'static #message_ident>( self.borrowed.#field_ident.as_ref().unwrap() ) };

                                Some(#ref_ident {
                                    owner: self.owner.clone(),
                                    borrowed,
                                })
                            }
                        }
                    },
                    Cardinality::Repeated => {
                        // TODO: use proxy list
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> ::fastproto_lib::list::ProxyList {
                                let borrowed = unsafe { std::mem::transmute::<&'_[#message_ident], &'static[#message_ident]>( self.borrowed.#field_ident.as_slice() ) };

                                ::fastproto_lib::list::ProxyList(Box::new(::fastproto_lib::list::BorrowedList {
                                    owner: self.owner.clone(),
                                    slice: borrowed,
                                }))
                            }
                        }
                    }
                }
            }
        }
    });

    let ident_str = ident.to_string();
    quote::quote! {
        #[::pyo3::pyclass(name=#ident_str)]
        #[derive(Clone)]
        pub struct #ref_ident {
            owner: ::std::sync::Arc<dyn Send + Sync>,
            borrowed: &'static #ident,
        }

        impl Default for #ref_ident {
            fn default() -> Self {
                <Self as ::fastproto_lib::Ref<#ident>>::new_owned(::std::sync::Arc::<#ident>::default())
            }
        }

        impl ::fastproto_lib::AsBorrowed for #ident {
            type Borrowed = #ref_ident;

            fn as_borrowed(&self, owner: ::std::sync::Arc<dyn Send + Sync>) -> Self::Borrowed {
                let borrowed = unsafe { std::mem::transmute::<&'_ #ident, &'static #ident>(self) };
                <Self::Borrowed as ::fastproto_lib::Ref<#ident>>::new(owner, borrowed)
            }
        }

        impl ::fastproto_lib::Ref<#ident> for #ref_ident {
            fn new(owner: ::std::sync::Arc<dyn Send + Sync>, borrowed: &'static #ident) -> Self {
                Self {
                    owner,
                    borrowed,
                }
            }

            fn new_owned(owner: ::std::sync::Arc<#ident>) -> Self {
                let borrowed = unsafe { std::mem::transmute::<&'_ #ident, &'static #ident>(owner.as_ref()) };
                Self {
                    owner,
                    borrowed,
                }
            }
        }

        #impl_proto_api

        #[::pyo3::pymethods]
        impl #ref_ident {
            #(
                #getters
            )*

        }
    }
}
