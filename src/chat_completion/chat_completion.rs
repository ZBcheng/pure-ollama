use super::{request::ChatCompletionRequest, response::ChatResponse};
use crate::{
    errors::OllamaError,
    response::{check_response_valid, OllamaResponse},
};

/// Generate the next message in a chat with a provided model. This is a streaming endpoint,
/// so there will be a series of responses. Streaming can be disabled using "stream": false.
/// The final response object will include statistics and additional data from the request.
pub async fn chat(
    request: ChatCompletionRequest,
) -> Result<OllamaResponse<ChatResponse>, OllamaError> {
    let url = "http://localhost:11434/api/chat";
    let resp = reqwest::Client::default()
        .post(url)
        .json(&request)
        .send()
        .await;

    let response = check_response_valid(resp).await?;
    Ok(response.into())
}
