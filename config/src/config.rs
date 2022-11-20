use confique::{toml::FormatOptions, Config};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// The configuration for the micro cli.
#[derive(Config)]
pub struct Conf {
    /// The git repository, which will be copied when a new microservice will be created through cli.
    #[config(
        env = "MICRO_TEMPLATE_LINK",
        default = "https://github.com/rust-micro/microservice-template"
    )]
    pub microservice_template_link: String,
}

impl Conf {
    /// Returns the Conf object as toml string.
    /// Wonderful to create a default config file.
    pub fn get_toml() -> String {
        let toml = confique::toml::template::<Conf>(FormatOptions::default());
        toml
    }

    pub fn load(path: &str) -> Result<Conf, String> {
        let r = Conf::builder().env().file(path).load();

        r.map_err(|e| format!("Error: {:?}", e))
    }

    pub fn create_config_file(path: &PathBuf) -> Result<String, String> {
        if path.exists() {
            Err("Config file already exists.".to_string())
        } else {
            let mut file = File::create(path).map_err(|e| format!("Error: {:?}", e))?;
            let toml = Conf::get_toml();
            file.write_all(toml.as_bytes())
                .map_err(|e| format!("Error: {:?}", e))?;
            Ok("Config file created.".to_string())
        }
    }
}
