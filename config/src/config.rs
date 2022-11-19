use confique::{toml::FormatOptions, Config};
use std::path::Path;

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
}
