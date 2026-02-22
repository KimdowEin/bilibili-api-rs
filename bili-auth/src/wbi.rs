//! wbi 签名

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Wbi {
    #[serde(alias = "img_url")]
    img: String,
    #[serde(alias = "sub_url")]
    sub: String,
}

const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29,
    28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25,
    54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

impl Wbi {
    /// 获取 wbi 签名
    pub fn mixin_key(&self) -> String {
        let key = format!("{}{}", self.img, self.sub);
        let key_bytes = key.as_bytes();

        MIXIN_KEY_ENC_TAB
            .into_iter()
            .take(32)
            .map(|idx| key_bytes.get(idx).copied().unwrap_or(b'!'))
            .map(|b| b as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mixin_key_test() {
        let wbi = Wbi {
            img: "7cd084941338484aae1ad9425b84077c".to_owned(),
            sub: "4932caff0ff746eab6f01bf08b70ac45".to_owned(),
        };
        let mixin_key = wbi.mixin_key();
        assert_eq!(mixin_key, "ea1db124af3c7062474693fa704f4ff8");
    }
}
