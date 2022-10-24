use std::io::{stdin, stdout, Write};

pub struct Cli {}

// TODO: think about if this should be an instance in app.rs or just a static thing...
impl Cli {
    pub fn read_config() -> (String, String, String) {
        let mut api_key = String::new();
        print!("Please enter your api key: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut api_key)
            .expect("Did not enter a correct string");
        if let Some('\n') = api_key.chars().next_back() {
            api_key.pop();
        }
        if let Some('\r') = api_key.chars().next_back() {
            api_key.pop();
        }

        let mut api_token = String::new();
        print!("Please enter your api token: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut api_token)
            .expect("Did not enter a correct string");
        if let Some('\n') = api_token.chars().next_back() {
            api_token.pop();
        }
        if let Some('\r') = api_token.chars().next_back() {
            api_token.pop();
        }

        let mut member_id = String::new();
        print!("Please enter your member id: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut member_id)
            .expect("Did not enter a correct string");
        if let Some('\n') = member_id.chars().next_back() {
            member_id.pop();
        }
        if let Some('\r') = member_id.chars().next_back() {
            member_id.pop();
        }

        (api_key, api_token, member_id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_exists_spec() {}
}
