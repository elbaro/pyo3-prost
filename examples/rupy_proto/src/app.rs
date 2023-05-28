#[::fastproto_macro::pyclass_for_prost_struct]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub profile_url: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "3")]
    pub follower_ids: ::prost::alloc::vec::Vec<i64>,
}
#[::fastproto_macro::pyclass_for_prost_struct]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tweet {
    #[prost(message, optional, tag = "4")]
    pub author: ::core::option::Option<User>,
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub created_timestamp: i64,
    #[prost(string, repeated, tag = "5")]
    pub mentions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
