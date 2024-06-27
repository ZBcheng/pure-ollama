#[cfg(test)]
mod tests {
    use crate::api::{completion::GenerateRequestBuilder, options::OptionsConstructor};

    #[test]
    fn test_generate_request_builder() {
        let _ = GenerateRequestBuilder::default()
            .keep_alive(1)
            .mirostat(1)
            .build();
        // let options = OptionsBuilder::default().temperature(value)
    }
}
