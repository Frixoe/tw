use clap::Command;
use tw::commands::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new("tw")
        .author("The Boogie Man")
        .about("Todo Wallpaper Manager:\nSet your todos as your wallpaper so this time you truly blame yourself")
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
