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
            #[pyo3(name = "decode")]  // avoid the name conflict with prost::Message
            pub fn decode_py(bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<Self> {
                let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                <#ident as ::prost::Message>::decode(bytes).map(|data| {
                    let owner = ::std::sync::Arc::new(data);
                    Self::new_owned(owner)
                }).map_err(|e| {
                    ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                })
            }

            pub fn decode_merge(&mut self, py: ::pyo3::Python, bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<()> {
                // let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                // {
                //     let mut obj_mut = ::std::sync::Arc::get_mut(&mut self.0).ok_or_else(|| ::pyo3::exceptions::PyRuntimeError::new_err("You cannot mutate the object while borrowing."))?;
                //     <#ident as ::prost::Message>::merge(::core::ops::DerefMut::deref_mut(&mut obj_mut), bytes).map_err(|e| {
                //         ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                //     })?;
                // }
                Err(::pyo3::exceptions::PyRuntimeError::new_err("todo"))
                // Ok(())
            }

            #[pyo3(name = "encode")]
            pub fn encode_py<'a>(&self, py: ::pyo3::Python<'a>) -> ::pyo3::PyResult<&'a ::pyo3::types::PyBytes> {
                Ok(::pyo3::types::PyBytes::new_with(py, ::prost::Message::encoded_len(self.borrowed), |mut py_buf: &mut [u8]| {
                    ::prost::Message::encode(self.borrowed, &mut py_buf).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                    Ok(())
                })?)
            }

            pub fn clear(&mut self) -> ::pyo3::PyResult<()> {
                // let mut r = ::std::sync::Arc::get_mut(&mut self.0).ok_or_else(|| ::pyo3::exceptions::PyRuntimeError::new_err("You cannot mutate the object while borrowing."))?;
                // *r = Default::default();
                Err(::pyo3::exceptions::PyRuntimeError::new_err("todo"))
                // Ok(())
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
                            pub fn #field_ident(&self, py: ::pyo3::Python) -> ::core::option::Option<#ref_ident> {
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
                        quote::quote! {

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
                Self::new_owned(Default::default())
            }
        }

        impl #ref_ident {
            pub fn new(owner: ::std::sync::Arc<#ident>, borrowed: &'static #ident) -> Self {
                Self {
                    owner,
                    borrowed,
                }
            }

            pub fn new_owned(owner: ::std::sync::Arc<#ident>) -> Self {
                let borrowed = unsafe { std::mem::transmute::<&'_ #ident, &'static #ident>(owner.as_ref()) };
                Self {
                    owner: owner as ::std::sync::Arc<dyn Send + Sync>,
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
