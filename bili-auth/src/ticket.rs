//! ticket 签名

use std::time::{SystemTime, UNIX_EPOCH};

use bili_core::{Data, ToQuery};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::wbi::Wbi;

pub const KEY_ID: &str = "ec02";

pub const KEY: &[u8; 9] = b"XgwSnGZ1p";

pub const BILI_TICKET_URL: &str =
    "https://api.bilibili.com/bapis/bilibili.api.ticket.v1.Ticket/GenWebTicket";

/// post
///
/// csrf
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery)]
pub struct BiliTicketQuery {
    // ec02
    pub key_id: String,
    pub hexsign: String,
    #[serde(rename = "context[ts]")]
    pub context: u64,
}

impl BiliTicketQuery {
    pub fn new() -> Self {
        let key_id = KEY_ID.to_string();

        let mut mac = Hmac::<Sha256>::new_from_slice(KEY).expect("can't be panic");

        let context = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("can't be panic")
            .as_secs();

        let message = format!("ts{}", context);
        mac.update(message.as_bytes());
        let result = mac.finalize();

        let hexsign = hex::encode(result.into_bytes());

        Self {
            key_id,
            hexsign,
            context,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Data)]
pub struct BiliTicket {
    pub ticket: String,
    pub created_at: u64,
    pub ttl: u64,
    #[serde(rename = "nav")]
    pub wbi: Wbi,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bili_core::{BiliResponse, ToQuery};
    use bili_service::{Session, SessionState};
    use reqwest::ClientBuilder;

    use crate::ticket::{BILI_TICKET_URL, BiliTicket, BiliTicketQuery};

    #[tokio::test]
    #[ignore = "error"]
    async fn test_get_ticket() {
        let state = SessionState::from_path("../cookies.json")
            .map(Arc::new)
            .unwrap();
        let client = ClientBuilder::new()
            .cookie_provider(state.store.clone())
            .build()
            .unwrap();
        let session = Session::new(client, state);

        let url = BiliTicketQuery::new()
            .to_query()
            .unwrap()
            .with_csrf(&session.bili_jct())
            .unwrap()
            .to_url(BILI_TICKET_URL);

        let json = session
            .post(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        tokio::fs::write("../tests/datas/auth_ticket.json", &json)
            .await
            .unwrap();

        let ticket = serde_json::from_str::<BiliResponse<BiliTicket>>(&json)
            .unwrap()
            .data()
            .unwrap();

        session.set_ticket(&ticket.ticket).unwrap();

        // eprintln!("{}", ticket.ticket);
    }
}
