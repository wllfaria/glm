use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    path: Option<String>,
}

pub fn parse() -> String {
    let args = Cli::parse();
    args.path.as_deref().unwrap_or(".").to_string()
}
