// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AntelopeBlockMeta {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub producer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub current_schedule: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub timestamp: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
