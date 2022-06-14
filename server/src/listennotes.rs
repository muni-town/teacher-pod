pub fn api_key() -> Option<&'static str> {
    let mock_api = std::env::var("MOCK_API").ok()?;
    if mock_api.to_lowercase() == "true" {
        return None;
    }
    let api_key = std::env::var("LISTEN_NOTES_KEY").ok()?;
    Some(Box::leak(Box::new(api_key)))
}