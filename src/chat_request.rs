use crate::ZukiInterface;
use serde::Serialize;
use serde_json::Value;


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
    pub async fn chat_request(
        &self,
        messages: Vec<Message>,
        model: &str,
        temperature: Option<f32>,
        unfiltered: bool,
    ) -> serde_json::Value {
        let temperature = temperature.unwrap_or(0.7);

        let body = ChatRequestBody {
            messages,
            model: model.to_string(),
            temperature,
        };

        let base_extension = if unfiltered { "unf" } else { "v1" };
        let url = format!("{}/{}/chat/completions", self.base_url, base_extension);

        let response = reqwest::Client::new()
            .post(&url)
            .header("Authorization", &self.auth)
            .json(&body)
            .send()
            .await
            .unwrap();

        // Parse response body as JSON
        let json_response: Value = response.json().await.unwrap();

        // Safely extract the content
        if let Some(content) = json_response["choices"]
            .get(0)
            .and_then(|choice| choice["message"]["content"].as_str())
        {
            return content.to_string().into();
        }

        panic!("No content found in the response");
    }
}