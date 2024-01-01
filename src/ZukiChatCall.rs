use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatData {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

pub struct ZukiChatCall<'a> {
    pub api_key: &'a str,
}

impl ZukiChatCall<'_> {
    pub fn new(api_key: &str) -> ZukiChatCall {
        ZukiChatCall { api_key: api_key}
    }

    pub fn chat_data(&self, user_name: &str, user_message: &str, requested_model: &str, system_prompt: &str, curr_temp: f64) -> ChatData {
        let data = ChatData {
            model: requested_model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!(
                        "{}\n Here is a message a user called {} sent you: {}",
                        system_prompt, user_name, user_message
                    ),
                },
            ],
            temperature: curr_temp,
        };

        data
    }

    pub async fn chat_call(&self, user_name: &str, user_message: &str, requested_model: &str, system_prompt: &str, curr_temp: f64, endpoint: &str) -> Result<String, reqwest::Error> {
        let data = self.chat_data(user_name, user_message, requested_model, system_prompt, curr_temp);

        let response = reqwest::Client::new()
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&data)
            .send()
            .await?;

        let response_data: serde_json::Value = response.json().await?; // Change the type to match the actual response structure

        Ok(response_data["choices"][0]["message"]["content"].as_str().unwrap_or_default().to_string())
    }
}