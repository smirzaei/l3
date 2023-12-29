use clap::Parser;

#[derive(Debug, Parser)]
#[command(author)]
pub struct Args {
    // Path to the config file
    #[arg(short, long)]
    pub config: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
