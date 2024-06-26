#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zp2p {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// for p2p
    #[prost(enumeration = "ZIdentity", tag = "2")]
    pub r#type: i32,
    /// for p2p
    #[prost(enumeration = "ZAction", tag = "3")]
    pub action: i32,
    /// for vlc
    #[prost(enumeration = "ZPushType", tag = "4")]
    pub push_type: i32,
    #[prost(message, optional, tag = "5")]
    pub message: ::core::option::Option<ZMessage>,
    #[prost(bytes = "vec", tag = "6")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    /// for verifying
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// ZMessage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub version: u32,
    #[prost(enumeration = "ZType", tag = "3")]
    pub r#type: i32,
    #[prost(enumeration = "ZAction", tag = "4")]
    pub action: i32,
    #[prost(enumeration = "ZIdentity", tag = "5")]
    pub identity: i32,
    #[prost(bytes = "vec", tag = "6")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "10")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZChat {
    #[prost(string, tag = "1")]
    pub message_data: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub clock: ::core::option::Option<ClockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeLog {
    #[prost(bytes = "vec", tag = "1")]
    pub from_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub to_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub start_count: u64,
    #[prost(uint64, tag = "4")]
    pub end_count: u64,
    #[prost(message, optional, tag = "5")]
    pub s_clock: ::core::option::Option<Clock>,
    #[prost(message, optional, tag = "6")]
    pub e_clock: ::core::option::Option<Clock>,
    #[prost(uint64, tag = "7")]
    pub merge_at: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(map = "string, uint64", tag = "1")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClockInfo {
    #[prost(message, optional, tag = "1")]
    pub clock: ::core::option::Option<Clock>,
    /// id 为node id
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub count: u64,
    #[prost(uint64, tag = "5")]
    pub create_at: u64,
}
/// type = TYPE_CLOCK_NODE
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClockNode {
    #[prost(message, optional, tag = "1")]
    pub clock: ::core::option::Option<Clock>,
    #[prost(bytes = "vec", tag = "2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub count: u64,
    #[prost(uint64, tag = "5")]
    pub create_at: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub raw_message: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    #[prost(string, repeated, tag = "1")]
    pub node_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZIdentity {
    /// client
    UTypeCli = 0,
    /// server
    UTypeSer = 1,
}
impl ZIdentity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ZIdentity::UTypeCli => "U_TYPE_CLI",
            ZIdentity::UTypeSer => "U_TYPE_SER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "U_TYPE_CLI" => Some(Self::UTypeCli),
            "U_TYPE_SER" => Some(Self::UTypeSer),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZAction {
    /// read
    ZTypeRead = 0,
    /// write
    ZTypeWrite = 1,
}
impl ZAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ZAction::ZTypeRead => "Z_TYPE_READ",
            ZAction::ZTypeWrite => "Z_TYPE_WRITE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Z_TYPE_READ" => Some(Self::ZTypeRead),
            "Z_TYPE_WRITE" => Some(Self::ZTypeWrite),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZPushType {
    /// direct msg
    ZTypeDm = 0,
    /// broadcast
    ZTypeBc = 1,
}
impl ZPushType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ZPushType::ZTypeDm => "Z_TYPE_DM",
            ZPushType::ZTypeBc => "Z_TYPE_BC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Z_TYPE_DM" => Some(Self::ZTypeDm),
            "Z_TYPE_BC" => Some(Self::ZTypeBc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZType {
    Rng = 0,
    Event = 1,
    Clock = 2,
    Gateway = 3,
    Zchat = 4,
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
            ZType::Gateway => "Z_TYPE_GATEWAY",
            ZType::Zchat => "Z_TYPE_ZCHAT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Z_TYPE_RNG" => Some(Self::Rng),
            "Z_TYPE_EVENT" => Some(Self::Event),
            "Z_TYPE_CLOCK" => Some(Self::Clock),
            "Z_TYPE_GATEWAY" => Some(Self::Gateway),
            "Z_TYPE_ZCHAT" => Some(Self::Zchat),
            _ => None,
        }
    }
}
