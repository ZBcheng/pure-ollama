#[cfg(test)]
mod tests {
    use tokio_stream::StreamExt;

    use crate::{
        completion::{
            completion::completion, request::CompletionRequestBuilder, response::GenerateResponse,
        },
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
        match response {
            GenerateResponse::Stream(_) => panic!("unexpected response type: stream"),
            GenerateResponse::NonStream(resp) => println!("resp: {:?}", resp),
        }
    }

    #[ignore]
    #[tokio::test]
    async fn test_generate_stream() {
        let request = CompletionRequestBuilder::default()
            .model("llama3:8b")
            .prompt("good morning")
            .build()
            .unwrap();

        match completion(request).await.unwrap() {
            GenerateResponse::NonStream(_) => panic!("unexcepted type: non-stream"),
            GenerateResponse::Stream(mut s) => {
                while let Some(item) = s.next().await {
                    match item {
                        Ok(inner) => println!("inner: {:?}", inner),
                        Err(e) => panic!("{}", e),
                    }
                }
            }
        }
    }
}
