use clap::Command;
use tw::commands::run;
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let matches = Command::new("tw")
        .author("Suryansh S.<https://github.com/frixoe>")
        .about("Set your todos as your wallpaper so this time you truly blame yourself")
        .version("0.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
            .about("Parse the config, generate the image and set the wallpaper")
        ).get_matches();

    match matches.subcommand() {
        Some(("run", _)) => {
            let _ = run().await;
        }
        _ => unreachable!(),
    }

    Ok(())
}
