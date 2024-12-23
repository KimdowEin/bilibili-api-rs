use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct VipStat{
    pub mid: u64,
    pub vip_type: u8,
    pub vip_status: u8,
    pub vip_due_date: u64,
    pub vip_pay_type: bool,
    pub theme_type: u8,
}