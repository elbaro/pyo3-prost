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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tweet {
    #[prost(message, tag = "10")]
    pub user0: ::core::option::Option<User>,
    #[prost(message, required, tag = "1")]
    pub user1: User,
    #[prost(message, optional, tag = "2")]
    pub user2: ::core::option::Option<User>,
    // #[prost(message, repeated, optional, tag = "3")]
    // pub user3: ::prost::alloc::vec::Vec<User>,
    #[prost(string, tag = "4")]
    pub text1: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub text2: ::core::option::Option<::prost::alloc::string::String>,
    // #[prost(string, repeated, tag = "6")]
    // pub text3: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub i1: i64,
    #[prost(int64, optional, tag = "8")]
    pub i2: ::core::option::Option<i64>,
    // #[prost(int64, repeated, tag = "9")]
    // pub i3: ::prost::alloc::vec::Vec<i64>,
}
