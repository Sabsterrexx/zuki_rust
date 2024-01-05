mod ZukiCall;
mod ZukiChat;
mod ZukiChatCall;


#[cfg(test)]
mod tests {

    use crate::ZukiCall::ZukiCall;
    use crate::ZukiChat::ZukiChat;
    use crate::ZukiChatCall::ZukiChatCall;

    const API_KEY: &str = ""; //Set to the value of your actual API key.
    const CHAT_MODEL: &str = ""; //Choose whatever model you want.

    #[tokio::test]
    async fn chat_sub_call() {
        //Testing that intialization does not return an error:
        let zuki_chat_call: ZukiChatCall<'_> = ZukiChatCall { api_key: "" }; //Replace "" with API_KEY.

        match zuki_chat_call
            .chat_call("Sabs", "Hello, how are you?", "gpt-3.5", "", 0.5, "")
            .await
        {
            //Replace first "" with prompt (optional), replace last "" with API endpoint
            Ok(result) => {
                // Print the result here or do any further processing
                println!("API Result: {}", result);
            }
            Err(err) => {
                // Handle the error, e.g., print an error message
                eprintln!("Error: {}", err);
            }
        }
    }
    #[tokio::test]
    async fn chat_call() {
        let zuki_call: ZukiCall<'_> = ZukiCall::new(API_KEY, CHAT_MODEL);
        let message = zuki_call.zuki_chat.send_message("Sabs", "Hey, how are ya?").await;
        println!("Message: {:?}", message);
    }
}
