use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug,Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Quality {
    FL = 2,
    HD = 3,
    ART = 4,
}

#[derive(Debug,Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Qn{
    FL = 80,
    HD = 150,
    Blue = 400,
    ART = 10000,
    K4 = 20000,
    Dolby = 30000,
}