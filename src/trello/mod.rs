use std::env;

pub fn ApiConnector() {
    let apiKey = env::var("API_KEY").is_ok();
    let apiToken = env::var("API_TOKEN").is_ok();
    println!("{}", apiKey);
    println!("{}", apiToken);
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn read_env_api_key() {
        let apiKey = env::var("API_KEY").is_ok();
        assert_ne!(false, apiKey)
    }

    #[test]
    fn read_env_api_token() {
        let apiToken = env::var("API_TOKEN").is_ok();
        assert_ne!(false, apiToken)
    }
}
