use clap::{Parser, Subcommand};
use micro::{
    docker::check_requirements as docker_check,
    etcd::{
        check_requirements as etcd_check, cleanup as etcd_cleanup, start as etcd_start,
        stop as etcd_stop,
    },
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    _debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new project
    New {
        /// The name of the project. Taken for the foldername and servicename.
        #[arg(short, long)]
        name: String,
    },
    /// Checks the requirements to run the microservices with cargo-micro.
    Check,

    /// Installs and executes needed subsystems like etcd.
    Init,

    /// Stops all subsystems, which were started before with `init`.
    Stop,

    /// Removes all resources.
    Cleanup,

    /// Executes all configured microservices in the current project.
    Run,
}

pub async fn run() {
    let cli = Cli::parse();

    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Check => {
                println!("Checks requirements...");
                docker_check().await;
                etcd_check().await;
                println!("Everything is right in place. You can run `cargo micro run`.");
            }
            Commands::Init => {
                println!("Starts subsystems...");
                etcd_start().await;
                println!("Everything is up and running. You can run `cargo micro run`.");
            }
            Commands::Stop => {
                println!("Stops all subsystems...");
                etcd_stop().await;
                println!("Everything is stopped now.");
            }
            Commands::Cleanup => {
                println!("Starts cleanup...");
                etcd_cleanup().await;
                println!(
                    "Cleanup done. You can uninstall cargo-micro with: `cargo uninstall micro`."
                );
            }
            _ => {}
        }
    }
}
