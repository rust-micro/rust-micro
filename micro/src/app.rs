use args::{get_cli_arguments, Commands};
use config::Conf;
use std::fs::File;
use std::io::Write;

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

        if self.cli.config_file.is_file() {
            self.config = Conf::load(&self.cli.config_file.to_str().unwrap())
                .map_err(|e| println!("{}", e))
                .ok();
        }

        match &self.cli.command {
            Some(Commands::Init) => {
                let config_file = &self.cli.config_file;

                if config_file.exists() {
                    println!("Config file already exists.");
                } else {
                    let mut file = File::create(config_file).unwrap();
                    let toml = Conf::get_toml();
                    file.write_all(toml.as_bytes()).unwrap();
                    println!("Config file created.");
                }
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
            None => {
                println!("No command given.");
            }
        }
    }
}
