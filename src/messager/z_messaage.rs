#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub version: u32,
    #[prost(enumeration = "ZType", tag = "3")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZType {
    Rng = 0,
    Event = 1,
    Clock = 2,
}
impl ZType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ZType::Rng => "Z_TYPE_RNG",
            ZType::Event => "Z_TYPE_EVENT",
            ZType::Clock => "Z_TYPE_CLOCK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Z_TYPE_RNG" => Some(Self::Rng),
            "Z_TYPE_EVENT" => Some(Self::Event),
            "Z_TYPE_CLOCK" => Some(Self::Clock),
            _ => None,
        }
    }
}
