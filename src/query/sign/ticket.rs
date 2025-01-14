use std::time::{self, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{error::Error, traits::Query};

pub const BILI_TICKET_URL:&str = "https://api.bilibili.com/bapis/bilibili.api.ticket.v1.Ticket/GenWebTicket";


#[derive(Debug,Serialize,Deserialize)]
pub struct BiliTicketQuery{
    // ec02
    key_id:String,
    hexsign:String,
    #[serde(rename = "context[ts]")]
    context:u64,
    csrf:String,
}
impl Query for BiliTicketQuery  {}
impl BiliTicketQuery {
    pub fn new(csrf:String)->Result<Self,Error>{
        let key_id = "ec02".to_string();
        let context = time::SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let key = b"XgwSnGZ1p";
        let message = format!("ts{}", context);
        let mut mac:Hmac<Sha256> = Hmac::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();

        let hexsign = hex::encode(result.into_bytes());
        Ok(Self {
            key_id,
            hexsign,
            context,
            csrf,
        })
    }
}