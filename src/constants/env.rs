pub fn whitelisted_users() -> Vec<String> {
    std::env::var("WHITELISTED_USERS")
        .unwrap_or_else(|_| "123456789".to_string()) // Default to a dummy user ID
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}
