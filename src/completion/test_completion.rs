#[cfg(test)]
mod tests {
    use tokio::io::{stdout, AsyncWriteExt};
    use tokio_stream::StreamExt;

    use crate::{
        completion::{completion::completion, request::CompletionRequestBuilder},
        errors::OllamaError,
        options::OptionsConstructor,
    };

    #[test]
    fn test_generate_request() {
        let request = CompletionRequestBuilder::default()
            .model("llama3")
            .prompt("hello")
            .num_predict(1)
            .build()
            .unwrap();

        let serialized = serde_json::to_string(&request).unwrap();
        println!("serialized: {}", serialized);
    }

    #[ignore]
    #[tokio::test]
    async fn test_generate_non_stream() {
        let request = CompletionRequestBuilder::default()
            .model("llama3:8b")
            .prompt("good morning")
            .stream(false)
            .build()
            .unwrap();

        let response = completion(request).await.unwrap();
        let response = response.as_response().await.unwrap();
        dbg!("{}", response);
    }

    #[ignore]
    #[tokio::test]
    async fn test_generate_stream() {
        let request = CompletionRequestBuilder::default()
            .model("llama3:8b")
            .prompt("good morning")
            .build()
            .unwrap();

        let response = completion(request).await.unwrap();
        let mut stream = response.as_stream().await.unwrap();
        let mut out = stdout();
        while let Some(item) = stream.next().await {
            out.write(item.unwrap().response.as_bytes()).await.unwrap();
            out.flush().await.unwrap();
        }

        out.write(b"\n").await.unwrap();
        out.flush().await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn test_generate_stream_error() {
        let request = CompletionRequestBuilder::default()
            .model("unknown model")
            .prompt("anyway")
            .build()
            .unwrap();

        let response = completion(request).await.err().unwrap();
        let err_msg =
            String::from("{\"error\":\"model 'unknown model' not found, try pulling it first\"}");
        assert_eq!(response, OllamaError::OllamaError(err_msg));
    }
}
