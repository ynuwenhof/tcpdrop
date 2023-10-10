use clap::Parser;
use std::path::PathBuf;

mod tcp;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "TCPDROP_PRETTY")]
    pretty: bool,
    #[arg(long, env = "TCPDROP_NO_V4")]
    no_v4: bool,
    #[arg(long, env = "TCPDROP_NO_V6")]
    no_v6: bool,
    #[arg(short, long, env = "TCPDROP_OUTPUT", value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
