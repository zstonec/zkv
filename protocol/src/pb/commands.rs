/// 来自客户端的命令请求
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(oneof="command_request::RequestData", tags="1, 2, 3")]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag="1")]
        Get(super::Get),
        #[prost(message, tag="2")]
        Set(super::Set),
        #[prost(message, tag="3")]
        Del(super::Del),
    }
}
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Value>,
    #[prost(uint32, optional, tag="3")]
    pub ttl: ::core::option::Option<u32>,
}
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Del {
    #[prost(string, repeated, tag="1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag="1")]
    pub status: u32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    #[prost(message, repeated, tag="4")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
}
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag="3")]
        Integer(i64),
        #[prost(double, tag="4")]
        Float(f64),
        #[prost(bool, tag="5")]
        Bool(bool),
    }
}
/// 返回的 kvPair
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Value>,
}
