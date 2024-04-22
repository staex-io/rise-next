use std::str::from_utf8;

use clap::Parser;

/// Command line utility to interact with NEXA streaming.
#[derive(Parser)]
#[clap(name = "streaming")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set input for ffmpeg (ex: -i /rec/drone.flv).
    #[arg(short, long)]
    #[arg(default_value = "rec/drone.flv")]
    input: String,
    /// Set output path where you want to save image.
    #[arg(short, long)]
    #[arg(default_value = "last.jpg")]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(format!("-sseof -3 -i {} -update 1 -q:v 1 {}", cli.input, cli.output).split(' '));
    let output = cmd.output().unwrap();
    if !output.status.success() {
        eprintln!("Exit status is not success: {}", output.status);
        eprintln!("There are logs from output:");
        eprintln!("{:?}", from_utf8(&output.stdout));
    }
}
