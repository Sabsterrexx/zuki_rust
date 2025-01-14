use crate::ZukiInterface;
use serde::Serialize;

#[derive(Serialize)]
pub struct ImageData {
    prompt: String,
    model: String,
    size: String,
    quality: String,
    n: u8
}

impl ZukiInterface {
    pub async fn image_call(&self, model: &str, prompt: &str, size: &str, quality: &str, n: u8) -> serde_json::Value {
        let body = ImageData {
            prompt: String::from(prompt),
            model: String::from(model),
            size: String::from(size),
            quality: String::from(quality),
            n
        };
        
        let base_extension = "v1";
        let url = format!("{}/{}/images/generations", self.base_url, base_extension);

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