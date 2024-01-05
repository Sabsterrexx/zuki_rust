use crate::ZukiChat::ZukiChat;

pub struct ZukiCall<'a> {
    api_key: String,
    chat_model: String,
    pub zuki_chat: ZukiChat<'a>,
}

impl ZukiCall<'_> {
    pub fn new<'a>(api_key: &'a str, chat_model: &'a str) -> ZukiCall<'a> {
        let zuki_chat = ZukiChat::new(api_key, chat_model, "You are a helpful assistant.", 0.7);

        ZukiCall {
            api_key: api_key.to_string(),
            chat_model: chat_model.to_string(),
            zuki_chat,
        }
    }
}
