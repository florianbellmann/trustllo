use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use super::Config;
pub struct ConfigManager {}

//TODO:  should this also be an instance instead of static?
impl ConfigManager {
    // TODO: remove my name
    const DEFAULT_CONFIG_LOCATION: &str = "/Users/florian.juesten/.config/trustllo/config.json";
    // &std::env::var("HOME").unwrap().to_owned();
    // Ok(path) => "fdjskl",
    // Err(e) => "~/.config/trustllo/config.json",
    // };

    pub fn config_exists(custom_path: Option<&str>) -> bool {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);
        // println!("{:?}", config_path);
        // println!("{:?}", Path::new(config_path).is_file());

        if !Path::new(config_path).is_file() {
            return false;
        }

        let mut file = File::open(config_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents);
        let config: Config = match serde_json::from_str(&file_contents) {
            Ok(file) => file,
            Err(_e) => return false,
        };

        !config.api_key.is_empty() && !config.api_token.is_empty() && !config.member_id.is_empty()
    }

    pub fn create_config(
        api_key: String,
        api_token: String,
        member_id: String,
        custom_path: Option<&str>,
    ) {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);

        let file_exists = Path::new(config_path).is_file();

        if file_exists {
            println!("Config file already exists");
        } else {
            let config: Config = Config {
                api_key,
                api_token,
                member_id,
            };
            let config_string = serde_json::to_string(&config).unwrap();

            println!("Config with contents {} created.", &config_string);
            fs::write(config_path, config_string);
        }
    }

    pub fn remove_config(custom_path: Option<&str>) {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);
        fs::remove_file(config_path);
    }

    pub fn read_config(custom_path: Option<&str>) -> Result<Config, Box<dyn Error>> {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);

        let mut file = File::open(config_path)?;
        let mut file_contents = String::new();
        // file.read_to_string(&mut file_contents)?;
        file.read_to_string(&mut file_contents);

        let config: Config = serde_json::from_str(&file_contents).unwrap();

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        path::Path,
    };

    use crate::utils::types::get_type_of;

    use super::ConfigManager;

    #[test]
    fn config_exists_spec() {
        let non_existant_config_name: &str = "/tmp/trustllo_nonexistant_config.json";
        let custom_config_name: &str = "/tmp/trustllo_custom_config.json";

        // check default config exists, as a dev I always have one present.
        let default_config_exists = ConfigManager::config_exists(None);
        assert!(default_config_exists);

        // check that unexisting config is not existing
        let non_existant_config_does_not_exist =
            ConfigManager::config_exists(Some(non_existant_config_name));
        assert!(!non_existant_config_does_not_exist);

        // create custom config and check it's existance. Should be false because it's not a valid config
        let _custom_config = fs::write(custom_config_name, b"Hello world!");
        let custom_config_exists_but_wrong = ConfigManager::config_exists(Some(custom_config_name));
        assert!(!custom_config_exists_but_wrong);

        // custom config exists and is valid
        fs::write(
            custom_config_name,
            b"{api_key:123, api_token:456, member_id:789}",
        );
        let custom_config_exists_but_correct =
            ConfigManager::config_exists(Some(custom_config_name));
        assert!(!custom_config_exists_but_correct);

        // remove custom config
        fs::remove_file(custom_config_name);
    }

    #[test]
    fn create_new_config_spec() {
        let new_config_name: &str = "/tmp/trustllo_new_config.json";

        ConfigManager::create_config(
            String::from("teSt_key"),
            String::from("teSt_token"),
            String::from("teSt_member_id"),
            Some(new_config_name),
        );
        assert_eq!(true, Path::new(new_config_name).is_file());

        let config = ConfigManager::read_config(Some(new_config_name)).unwrap();

        assert_eq!(config.api_key, "teSt_key");
        assert_eq!(config.api_token, "teSt_token");
        assert_eq!(config.member_id, "teSt_member_id");

        // remove the file
        fs::remove_file(new_config_name);
        assert_eq!(false, Path::new(new_config_name).is_file())
    }

    #[test]
    fn read_defaut_config_spec() {
        // read the default config
        let config = ConfigManager::read_config(None).unwrap();
        assert_eq!(get_type_of(&config), "trustllo::config::Config");
    }

    #[test]
    fn remove_config_spec() {
        // removes config file completely
        let remove_config_name = "/tmp/trustllo_config_to_remove.json";
        fs::write(
            remove_config_name,
            b"{api_key:123, api_token:456, member_id:789}",
        );
        assert!(Path::new(remove_config_name).is_file());

        ConfigManager::remove_config(Some(remove_config_name));
        assert!(!Path::new(remove_config_name).is_file());
    }
}
