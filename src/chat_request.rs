use crate::ZukiInterface;
use serde::Serialize;


#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}


#[derive(Serialize)]
struct ChatRequestBody {
    messages: Vec<Message>,
    model: String,
    temperature: f32,
}


impl ZukiInterface {
    pub async fn chat_request(&self, messages: Vec<Message>, model: String, temperature: Option<f32>, unfiltered: bool) -> serde_json::Value {
        let temperature = temperature.unwrap_or(0.7);

        let body = ChatRequestBody {
            messages,
            model,
            temperature
        };

        let base_extension = if unfiltered { "unf" } else { "v1" };
        let url = format!("{}/{}/chat/completions", self.base_url, base_extension);

        let request = reqwest::Client::new()
            .post(&url)
            .header("Authorization", &self.auth)
            .json(&body)
            .send()
            .await
            .unwrap();

        request.json().await.unwrap()
    }
}