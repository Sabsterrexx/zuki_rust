use zuki_rust::ZukiInterface;

#[tokio::main]
async fn main() {
    let client = ZukiInterface::new(""); //Put your API key here.

    let messages = vec![zuki_rust::chat_call::Message {
        role: String::from("user"),
        content: String::from("What's 9 + 10?"),
    }];

    let chat_response = client
        .chat_call(messages, "gpt-3.5-turbo", None, false)
        .await; //Make the chat request

    let content = chat_response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap(); //Parse it
    println!("{}", content); //Print the chat message

    let image_response = client
        .image_call("flux-schnell", "A plane", "512x512", "standard", 1) //Make the image request
        .await;
    // Parse the response to get only the image URL
    let image_url = image_response["data"][0]["url"]
        .as_str()
        .unwrap_or("No URL found");
    println!("{}", image_url); //Print the image url
}
