pub mod chat_request;
pub mod image_request;

pub struct ZukiInterface {
    auth: String,
    base_url: String
}

impl ZukiInterface {
    pub fn new(auth: &str) -> ZukiInterface {
        ZukiInterface {
            auth: String::from(auth),
            base_url: String::from("https://zukijourney.xyzbot.net")
        }
    }
}