mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let package_name = env!("CARGO_PKG_NAME");
    //
    // let version = format!(
    //     "{}.{}.{}{}",
    //     env!("CARGO_PKG_VERSION_MAJOR"),
    //     env!("CARGO_PKG_VERSION_MINOR"),
    //     env!("CARGO_PKG_VERSION_PATCH"),
    //     option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    // );
    //
    // println!(
    //     "The current {} package version is {}",
    //     package_name, version
    // );
    cli::run().await;
    Ok(())
}
