mod ZukiChatCall;

#[cfg(test)]
mod tests {
    use crate::ZukiChatCall::ZukiChatCall;
    #[tokio::test]
    async fn chat_sub_call(){

        //Testing that intialization does not return an error:

        let zuki_chat_call = ZukiChatCall{api_key: ""}; //Replace "" with API key.
        match  zuki_chat_call.chat_call("Sabs", "Hello, how are you?", "gpt-3.5-turbo", "", 0.5, "").await{ 
            
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
}
