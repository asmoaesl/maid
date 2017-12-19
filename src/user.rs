use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Read};
use std::env::current_exe;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub preferred_compiler: Compiler,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Compiler {
    GNU,
    Clang,
}

impl Config {
    /// Obtain the user config file, or create one with defaults if it does not exist.
    pub fn get() -> Result<Config, &'static str> {
        // Get the directory Maid is placed in
        let mut path: PathBuf;
        match current_exe().unwrap().parent() {
            Some(p) => path = p.to_owned(),
            // Very likely will not ever happen
            None    => return Err("Maid is not placed in a directory, so we cannot have a user configuration file."),
        }

        path = path.join("Config.toml");

        let config = if path.is_file() {
            // Open the config file
            let mut file: File;
            match File::open(path) {
                Ok(f) => file = f,
                Err(_) => return Err("Cannot open the configuration file."),
            }

            // Read the contents
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // Deserialize the TOML
            ::toml::from_str::<Config>(contents.as_str()).unwrap()
        } else {
            // Create the configuration file
            Config::new(path.as_path())?
        };

        Ok(config)
    }

    // This method is private because noone should be able to overwrite the config
    fn new(path: &Path) -> Result<Config, &'static str> {
        // Create the file
        let mut config_file: File;
        match File::create(path) {
            Ok(f) => config_file = f,
            Err(_) => return Err("Cannot create configuration file."),
        }

        // Initialize the configuration
        let config = Config {
            preferred_compiler: Compiler::GNU,
        };

        // Serialize the config into TOML
        let toml = ::toml::to_string(&config).unwrap();

        // Write the config to the new configuration file
        write!(config_file, "{}", toml).unwrap();
        // Sync IO operations for the new file before continuing
        config_file.sync_all().unwrap();

        Ok(config)
    }
}