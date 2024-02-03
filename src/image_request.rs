use crate::ZukiInterface;

impl ZukiInterface {
    pub async fn image_request(&self, message: &str) -> String {
        let response = format!("ZukiInterface::image_request({})", message);
        response
    }
}