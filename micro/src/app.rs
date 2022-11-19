use args::{get_cli_arguments, Commands};
use config::Conf;
use std::path::PathBuf;

pub struct App {
    cli: args::CliArgs,
    config: Option<Conf>,
}

impl App {
    pub fn new() -> App {
        let cli = get_cli_arguments();
        App { cli, config: None }
    }

    pub fn run(&mut self) {
        if self.cli.debug > 0 {
            println!("Debugging information is on.");
        }

        if self.cli.debug >= 1 {
            println!("Used config file: {}", self.cli.config_file.display());
        }

        if self.cli.config_file.is_file() {
            self.config = Conf::load(&self.cli.config_file.to_str().unwrap())
                .map_err(|e| println!("{}", e))
                .ok();
        }

        match &self.cli.command {
            Some(Commands::Init) => {
                let path = match option_env!("MICRO_CONFIG_FILE") {
                    Some(path) => PathBuf::from(path),
                    None => self.cli.config_file.clone(),
                };

                Conf::create_config_file(&path)
                    .map_err(|e| eprintln!("{}", e))
                    .ok();
            }
            Some(Commands::New { name }) => match &self.config {
                Some(config) => {
                    println!("Creating new microservice with name {}.", name);
                    println!("Using template link {}.", config.microservice_template_link);
                }
                None => {
                    println!("No configuration file found.");
                }
            },
            None => {}
        }
    }
}
