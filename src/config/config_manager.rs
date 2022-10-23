use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use super::Config;
pub struct ConfigManager {}

impl ConfigManager {
    const DEFAULT_CONFIG_LOCATION: &str = "~/.config/trustllo/config.json";

    pub fn config_exists(custom_path: Option<&str>) -> bool {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);

        if !Path::new(config_path).exists() {
            return false;
        }


        DEBUG ME

        let read_file_result = fs::read(config_path);

        let configfile = match read_file_result {
            Ok(file) => file,
            Err(_e) => return false, // TODO: log error
        };

        println!("{:#?}", configfile);

        todo!("Still needs happy path");
        todo!("Check JSON Parsing");
    }

    pub fn create_config(custom_path: Option<&str>) {
        let config_path = custom_path.unwrap_or(ConfigManager::DEFAULT_CONFIG_LOCATION);

        let file_exists = Path::new(config_path).is_file();

        if file_exists {
            println!("Config file already exists");
        } else {
            fs::write(config_path, b"{}");
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

    use crate::{config::Config, utils::types::get_type_of};

    use super::ConfigManager;

    const TEST_CONFIG_NAME: &str = "/tmp/trustllo_custom_config";

    #[test]
    fn config_exists_spec() {
        // check default config exists, as a dev I always have one present.
        let default_config_exists = ConfigManager::config_exists(None);
        assert!(default_config_exists);

        // check that unexisting config is not existing
        let non_existant_config_does_not_exist =
            ConfigManager::config_exists(Some("/tmp/jkldfjsdlk/config.json"));
        assert!(!non_existant_config_does_not_exist);

        // create custom config and check it's existance. Should be false because it's not a valid config
        let custom_config = fs::write(TEST_CONFIG_NAME, b"Hello world!");

        let custom_config_exists_but_wrong = ConfigManager::config_exists(Some(TEST_CONFIG_NAME));
        assert!(!custom_config_exists_but_wrong);

        // custom config exists and is valid
        fs::write(TEST_CONFIG_NAME, b"{api_key:123, api_token:456}");
        let custom_config_exists_but_correct = ConfigManager::config_exists(Some(TEST_CONFIG_NAME));
        assert!(!custom_config_exists_but_correct);

        // remove custom config
        fs::remove_file(TEST_CONFIG_NAME);
    }

    #[test]
    fn create_new_config_spec() {
        ConfigManager::create_config(Some(TEST_CONFIG_NAME));
        assert_eq!(true, Path::new(TEST_CONFIG_NAME).is_file())
    }

    #[test]
    fn read_config_spec() {
        // read the default config
        let config = ConfigManager::read_config(None).unwrap();
        assert_eq!(get_type_of(&config), "trustllo::config::Config");
        // TODO: this isn't really testing the parsing. just the return type. I need to add parsing
        // later and test that also
    }

    #[test]
    fn delete_config_spec() {
        // removes config file completely
        let config_path = "/tmp/trustllo_config_to_remove.json";
        let new_config = ConfigManager::create_config(Some(config_path));
        assert!(Path::new(config_path).is_file());

        ConfigManager::read_config(Some(config_path));
        assert!(!Path::new(config_path).is_file());
    }
}

// TODO: remove this dev commment
// parametrize tests!

// macro_rules! fib_tests {
//     ($($name:ident: $value:expr,)*) => {
//     $(
//         #[test]
//         fn $name() {
//             let (input, expected) = $value;
//             assert_eq!(expected, fib(input));
//         }
//     )*
//     }
// }

// fib_tests! {
//     fib_0: (0, 0),
//     fib_1: (1, 1),
//     fib_2: (2, 1),
//     fib_3: (3, 2),
//     fib_4: (4, 3),
//     fib_5: (5, 5),
//     fib_6: (6, 8),
// }
