use crate::ZukiChatCall::ZukiChatCall;

pub struct ZukiChat<'a> {
    api_key: String,
    api_endpoint: String,
    api_endpoint_unfiltered: String,
    api_endpoint_backup: String,
    system_prompt: String,
    model: String,
    temperature: f64,
    api_caller: ZukiChatCall<'a>,
}

impl ZukiChat <'_>{
    pub fn new<'a>(api_key: &'a str, model: &'a str, system_prompt: &'a str, temperature: f64) -> ZukiChat<'a> {
        let api_endpoint = "https://zukijourney.xyzbot.net/v1/chat/completions";
        let api_endpoint_unfiltered = "https://zukijourney.xyzbot.net/unf/chat/completions";
        let api_endpoint_backup ="https://thirdparty.webraft.in/v1/chat/completions";

        let api_caller = ZukiChatCall::new(api_key);

        let models_list = ["gpt-3.5", "gpt-3.5-turbo", "gpt-3.5-4k", "gpt-3.5-16k", "gpt-4", "gpt-4-4k", "gpt-4-16k", "claude-2"];

        if models_list.contains(&model) {
            ZukiChat {
                api_key: api_key.to_string(),
                api_endpoint: api_endpoint.to_string(),
                api_endpoint_unfiltered: api_endpoint_unfiltered.to_string(),
                api_endpoint_backup: api_endpoint_backup.to_string(),
                system_prompt: system_prompt.to_string(),
                model: model.to_string(),
                temperature,
                api_caller,
            }
        } else {
            panic!("{} is not a valid text model!", model);
        }
    }

    pub fn set_system_prompt(&mut self, new_prompt: &str) {
        self.system_prompt = new_prompt.to_string();
    }

    pub fn set_temp(&mut self, new_temp: f64) {
        if (0.0..=1.0).contains(&new_temp) {
            self.temperature = new_temp;
        } else {
            panic!("Temperature must be a value between 0 and 1!");
        }
    }

    pub fn change_backup_endpoint(&mut self, new_endpoint: &str) {
        self.api_endpoint_backup = new_endpoint.to_string();
    }

    pub async fn send_message(&self, user_name: &str, user_message: &str) -> String {

        match self.api_caller.chat_call(user_name, user_message, &self.model, &self.system_prompt, self.temperature, &self.api_endpoint).await{ 
            
            //Replace first "" with prompt (optional), replace last "" with API endpoint

            Ok(result) => {
                // Return the result here or do any further processing
                return result;
            }
            Err(err) => {
                // Return the error, e.g., print an error message
                return err.to_string();
            }

        }
    }

    pub async fn send_unfiltered_message(&self, user_name: &str, user_message: &str) -> String {
        match self.api_caller.chat_call(user_name, user_message, &self.model, &self.system_prompt, self.temperature, &self.api_endpoint_unfiltered).await{ 
            
            //Replace first "" with prompt (optional), replace last "" with API endpoint

            Ok(result) => {
                // Return the result here or do any further processing
                return result;
            }
            Err(err) => {
                // Return the error, e.g., print an error message
                return err.to_string();
            }

        }
    }

    pub async fn send_backup_message(&self, user_name: &str, user_message: &str) -> String {
        match self.api_caller.chat_call(user_name, user_message, &self.model, &self.system_prompt, self.temperature, &self.api_endpoint_backup).await{ 
            
            //Replace first "" with prompt (optional), replace last "" with API endpoint

            Ok(result) => {
                // Return the result here or do any further processing
                return result;
            }
            Err(err) => {
                // Return the error, e.g., print an error message
                return err.to_string();
            }

        }
    }
}