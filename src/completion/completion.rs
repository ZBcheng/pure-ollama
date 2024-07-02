use super::{request::CompletionRequest, response::CompletionResponse};
use crate::{
    errors::OllamaError,
    response::{check_response_valid, OllamaResponse},
};

/// Generate a response for a given prompt with a provided model. This is a streaming endpoint,
/// so there will be a series of responses. The final response object will include statistics and
/// additional data from the request.
pub async fn completion(
    request: CompletionRequest,
) -> Result<OllamaResponse<CompletionResponse>, OllamaError> {
    let url = format!("http://localhost:11434/api/generate");
    let resp = reqwest::Client::default()
        .post(url)
        .json(&request)
        .send()
        .await;

    let response = check_response_valid(resp).await?;
    Ok(response.into())
}
