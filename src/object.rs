// ## Object or Option<Object>

// ```rs
// #[pyclass]
// struct TweetOwned(Arc<Tweet>);
// #[pymethods]
// impl TweetOwned{
//   #[getter]
//   fn text(self: &PyAny) -> &PyAny {
//     if self.0.text.is_none() { return None; }
//     let arc_self: Arc<Self> = self.0.clone();
//     TextBorrowed(
//       owner: arc_self as Arc<dyn Any>,
//       borrowed: std::mem::transmute::<&'_ Text, &'static Text>( &arc_self.text ),
//     ) to pyobject
//   }
// }

// struct Tweet {
//   text: Text
// }

// #[pyclass]
// struct TextBorrowed {
//     <!-- We type-erase Arc<Tweet> -->
//     owner: Arc<dyn Any>,
//     borrowed: &'this Text,
// }
// ```

use prost_types::field::Cardinality;
use quote::format_ident;
use syn::Ident;

use crate::parse::{Field, FieldType};

pub fn derive_object_owned(ident: &Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let owned_ident = format_ident!("{}Owned", ident);
    let impl_ = quote::quote! {
        #[::pyo3::pymethods]
        impl #owned_ident {
            #[new]
            pub fn new() -> Self {
                Self::default()
            }

            #[staticmethod]
            #[pyo3(name = "decode")]  // avoid the name conflict with prost::Message
            pub fn decode_py(bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<Self> {
                let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                <#ident as ::prost::Message>::decode(bytes).map(|data|
                    Self(::std::sync::Arc::new(data))
                ).map_err(|e| {
                    ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                })
            }

            pub fn decode_merge(&mut self, py: ::pyo3::Python, bytes: &::pyo3::types::PyBytes) -> ::pyo3::PyResult<()> {
                let bytes: &[u8] = ::pyo3::FromPyObject::extract(bytes)?;
                {
                    let mut obj_mut = ::std::sync::Arc::get_mut(&mut self.0).ok_or_else(|| ::pyo3::exceptions::PyRuntimeError::new_err("You cannot mutate the object while borrowing."))?;
                    <#ident as ::prost::Message>::merge(::core::ops::DerefMut::deref_mut(&mut obj_mut), bytes).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                }
                Ok(())
            }

            #[pyo3(name = "encode")]
            pub fn encode_py<'a>(&self, py: ::pyo3::Python<'a>) -> ::pyo3::PyResult<&'a ::pyo3::types::PyBytes> {
                Ok(::pyo3::types::PyBytes::new_with(py, ::prost::Message::encoded_len(self.0.as_ref()), |mut py_buf: &mut [u8]| {
                    ::prost::Message::encode(self.0.as_ref(), &mut py_buf).map_err(|e| {
                        ::pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))
                    })?;
                    Ok(())
                })?)
            }

            pub fn clear(&mut self) -> ::pyo3::PyResult<()> {
                let mut r = ::std::sync::Arc::get_mut(&mut self.0).ok_or_else(|| ::pyo3::exceptions::PyRuntimeError::new_err("You cannot mutate the object while borrowing."))?;
                *r = Default::default();
                Ok(())
            }

            fn __repr__(&self) -> String {
                format!("{:?}", self.0.as_ref())
            }
            fn __str__(&self) -> String {
                format!("{:#?}", self.0.as_ref())
            }
        }




    };
    let getters = fields.iter().map(|field| {
        let field_ident = &field.ident;

        match &field.type_kind {
            FieldType::Map => {
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
                                self.0.#field_ident.clone()
                            }
                        }
                    }
                    Cardinality::Repeated => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> #field_ty {
                                self.0.#field_ident.clone()
                            }
                        }
                    }
                }
            }
            FieldType::Message => {
                let message_ident = field.message_ident.as_ref().unwrap();
                let borrowed_ident = format_ident!("{}Borrowed", message_ident);

                match field.cardinality {
                    Cardinality::Unknown => unreachable!(),
                    Cardinality::Required => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self) -> #borrowed_ident {
                                let arc: ::std::sync::Arc<#ident> = self.0.clone();
                                let borrowed = unsafe { std::mem::transmute::<&'_ #message_ident, &'static #message_ident>( &arc.#field_ident ) };

                                #borrowed_ident {
                                    owner: arc as ::std::sync::Arc<dyn ::std::any::Any + Send + Sync>,
                                    borrowed,
                                }

                            }
                        }
                    },
                    Cardinality::Optional => {
                        quote::quote! {
                            #[getter]
                            pub fn #field_ident(&self, py: ::pyo3::Python) -> ::core::option::Option<#borrowed_ident> {
                                if self.0.#field_ident.is_none() { return None; }
                                let arc: ::std::sync::Arc<#ident> = self.0.clone();
                                let borrowed = unsafe { std::mem::transmute::<&'_ #message_ident, &'static #message_ident>( arc.#field_ident.as_ref().unwrap() ) };

                                
                                Some(#borrowed_ident {
                                    owner: arc as ::std::sync::Arc<dyn ::std::any::Any + Send + Sync>,
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

    quote::quote! {
        #[::pyo3::pyclass]
        #[derive(Default, Clone)]
        pub struct #owned_ident(::std::sync::Arc<#ident>);

        #impl_

        #[::pyo3::pymethods]
        impl #owned_ident {
            #(
                #getters
            )*

        }
    }
}
pub fn derive_object_borrowed(ident: &Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let borrowed_ident = format_ident!("{}Borrowed", ident);
    let getters = fields.iter().map(|_field| {
        quote::quote! {}
    });

    quote::quote! {
        #[::pyo3::pyclass]
        pub struct #borrowed_ident {
            owner: ::std::sync::Arc<dyn ::std::any::Any + Send + Sync>,
            borrowed: &'static #ident,
        }

        #[::pyo3::pymethods]
        impl #borrowed_ident {
            #(
                #getters
            )*
        }
    }
}
pub fn derive_object(ident: &Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let stream1 = derive_object_owned(ident, fields);
    let stream2 = derive_object_borrowed(ident, fields);
    quote::quote! {
        #stream1
        #stream2
    }
}
