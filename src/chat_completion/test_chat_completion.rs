#[cfg(test)]
mod tests {
    use tokio::io::{stdout, AsyncWriteExt};
    use tokio_stream::StreamExt;

    use crate::chat_completion::{
        chat,
        message::{MessageBuilder, MessageBuilderError, Role},
        request::ChatCompletionRequestBuilder,
    };

    #[test]
    fn test_role() {
        let system = serde_json::to_string(&Role::System).unwrap();
        let assistant = serde_json::to_string(&Role::Assistant).unwrap();
        let user = serde_json::to_string(&Role::User).unwrap();

        assert_eq!(system, "\"system\"");
        assert_eq!(assistant, "\"assistant\"");
        assert_eq!(user, "\"user\"")
    }

    #[test]
    fn test_message() {
        let message = MessageBuilder::default()
            .content("hello")
            .role(Role::User)
            .build()
            .unwrap();

        let serialized = serde_json::to_string(&message).unwrap();
        let expect = "{\"role\":\"user\",\"content\":\"hello\"}";
        assert_eq!(&serialized, expect);

        let message = MessageBuilder::default()
            .content("world")
            .role("assistant")
            .images(vec![String::from("test")])
            .build()
            .unwrap();

        let serialized = serde_json::to_string(&message).unwrap();
        let expect = "{\"role\":\"assistant\",\"content\":\"world\",\"images\":[\"test\"]}";
        assert_eq!(&serialized, expect);

        let resp = MessageBuilder::default().content("world").build();
        assert_eq!(resp, Err(MessageBuilderError::UninitializedField("role")));
    }

    #[ignore]
    #[tokio::test]
    async fn test_chat_non_stream() {
        let request = ChatCompletionRequestBuilder::default()
            .model("llama3:8b")
            .stream(false)
            .messages(vec![MessageBuilder::default()
                .role(Role::User)
                .content("Hello")
                .build()
                .unwrap()])
            .build()
            .unwrap();

        let response = chat(request).await.unwrap();
        let response = response.response().await.unwrap();
        let resp_str = serde_json::to_string(&response).unwrap();
        dbg!("response: {:?}", resp_str);
    }

    #[ignore]
    #[tokio::test]
    async fn test_chat_stream() {
        let messages = vec![MessageBuilder::default()
            .role("user")
            .content("Hey there")
            .build()
            .unwrap()];

        let request = ChatCompletionRequestBuilder::default()
            .model("llama3:8b")
            .messages(messages)
            .build()
            .unwrap();

        let resp = chat(request).await.unwrap();
        let mut resp_stream = resp.as_stream().await.unwrap();

        let mut out = stdout();
        while let Some(item) = resp_stream.next().await {
            out.write(item.unwrap().message.unwrap().content.as_bytes())
                .await
                .unwrap();
            out.flush().await.unwrap();
        }
        out.write(b"\n").await.unwrap();
        out.flush().await.unwrap();
    }
}
