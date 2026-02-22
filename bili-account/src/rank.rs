use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum AccountPowerRank {
    NewUser = 5000,
    Normal = 10000,
    SubtitleMan = 20000,
    VIP = 25000,
    Official = 30000,
    Admin = 32000,

    #[serde(other)]
    Unknown,
}
