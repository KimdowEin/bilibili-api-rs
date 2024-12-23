use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wbi {
    img_url: String,
    sub_url: String,
}

const MIXIN_KEY_ENC_TAB: [u8; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29,
    28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25,
    54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];


impl Wbi {
    /// 获取 wbi 签名
    pub fn mixin_key(&self) -> String {
        let mut raw_wbi = String::new();
        raw_wbi.push_str(&self.img_url);
        raw_wbi.push_str(&self.sub_url);
        let binding = MIXIN_KEY_ENC_TAB
            .iter()
            .map(|&x| raw_wbi.as_bytes()[x as usize])
            .collect::<Vec<u8>>();
        let mut mixin_key = unsafe { String::from_utf8_unchecked(binding) };
        let _ = mixin_key.split_off(32); // 截取前 32 位字符
        mixin_key
    }
}



#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn mixin_key_test() {
        let wbi = Wbi{
            img_url: "7cd084941338484aae1ad9425b84077c".to_owned(),
            sub_url: "4932caff0ff746eab6f01bf08b70ac45".to_owned(),
        };
        let mixin_key = wbi.mixin_key();
        assert_eq!(mixin_key, "ea1db124af3c7062474693fa704f4ff8");
    }
}
