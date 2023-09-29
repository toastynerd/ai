use std::env;
pub fn send_to_chatgpt(prompt: String) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let mut client = reqwest::blocking::Client::new();
    let url = "https://api.openai.com/v1/engines/davinci/completions";
    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(format!("{{\"prompt\": \"{}\", \"max_tokens\": 200, \"temperature\": 0.9, \"top_p\": 1, \"frequency_penalty\": 0, \"presence_penalty\": 0}}", prompt))
        .send();
    if response.is_err() {
        return "Error sending request to OpenAI".to_string();
    }
    let response = response.unwrap();
    return response.text().unwrap();
}
