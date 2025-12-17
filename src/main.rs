use clap::Parser;
use fekit_cli::{config::Args, match_command};

fn main() {
    let args = Args::parse();

    match_command(&args);
}
