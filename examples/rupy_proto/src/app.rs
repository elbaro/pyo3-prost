#[::pyo3_prost::pyclass_for_prost_struct]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub profile_url: ::prost::alloc::string::String,
}
#[::pyo3_prost::pyclass_for_prost_struct]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tweet {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub created_timestamp: i64,
    #[prost(message, optional, tag="4")]
    pub author: ::core::option::Option<User>,
    #[prost(string, repeated, tag="5")]
    pub mentions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
