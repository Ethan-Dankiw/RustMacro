use std::io::{stdin, stdout, Write};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub mouse_input: String,
    pub keyboard_input: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        println!("\nAttempting to load config.toml");

        // Get the path to the config file
        let path = Self::get_config_path()?;

        // Print the config path
        println!("Looking for config.toml at the path: {}", path.display());

        // If the config file exists at the path, and is for a file
        if path.exists() && path.is_file() {
            // Load the config from disk
            let config = Self::load_config(&path)?;

            // Validate the values in the config file
            Self::validate_entries(&config)?;

            // Return the valid config
            return Ok(config)
        }

        // If the path to the config file exists, or is not for a file, create a new config file
        let config = Self::create_config(&path)?;

        // Return the newly created config file
        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf> {
        // Get the path to the current working directory where the executable is being run
        let cwd = std::env::current_dir()
            .context("Failed to get path to current working directory")?;

        // Return the path to the config file
        Ok(cwd.join("config.toml"))
    }

    fn load_config(path: &PathBuf) -> Result<Self> {
        println!("Detected config.toml file, loading from disk...");

        // Get the contents of the config file
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file {}", path.display()))?;

        // Parse the config file using the toml format
        let config: Config = toml::from_str(&contents).context("Failed to parse config.toml file")?;

        // Return the parsed config file
        Ok(config)
    }

    fn create_config(path: &PathBuf) -> Result<Self> {
        println!("No config.toml file detected, creating one...");

        // Print the user for input to populate the fields of the config
        let config = Self::prompt_user_for_config_entries()?;

        // Validate that the provided fields
        Self::validate_entries(&config)?;

        // Create a config.toml file from the inputs
        let toml = toml::to_string_pretty(&config).context("Failed to serialize config.toml")?;

        // Write the file to disk
        std::fs::write(path, toml).with_context(|| format!("Failed to write config.toml to {}", path.display()))?;

        // Return the created config file
        println!("Config file created: {}", path.display());
        Ok(config)
    }

    fn prompt_user_for_config_entries() -> Result<Self> {
        // Define the function used for prompting user input
        fn prompt(label: &str) -> Result<String> {
            // Prompt the user for input using the prompt label
            print!("{label}: ");
            stdout().flush()?;

            // Store the user's input in a string
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            // Return the user's input
            Ok(input.trim().to_string())
        }

        // Prompt the user for their mouse and keyboard event data
        let mouse = prompt("Mouse Input (e.g. event9)")?;
        let keyboard = prompt("Keyboard Input (e.g. event2)")?;

        // Return the user's config
        Ok(Self {
            mouse_input: mouse,
            keyboard_input: keyboard,
        })
    }

    fn validate_entries(config: &Self) -> Result<()> {
        for (name, value) in [
            ("mouse_input", &config.mouse_input),
            ("keyboard_input", &config.keyboard_input),
        ] {
            // If the value does not start with an event
            if !value.starts_with("event") {
                anyhow::bail!("{name} must look like 'eventX' to be a valid entry")
            }
        }

        // Indicate all entries are valid
        Ok(())
    }
}