use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name="J.Garnier", author, version, about, long_about = None)]
pub struct Cli {
    /// Audio file in absolute path
    #[arg(short, long, required = true)]
    pub input: PathBuf,

    /// Beat detection algorithm i.e ["Energy", "Hfc", "Complex", "Phase", "WPhase", "SpecDiff", "Kl", "Mkl", "SpecFlux"]
    #[arg(short, long, default_value = "SpecFlux")]
    pub beat_detection: String,

    /// Display static spectrum
    #[arg(short, long, conflicts_with_all=&["save_spectrum", "output", "live_visualization"])]
    pub visualization: bool,

    /// Play audio & display live spectrum
    #[arg(short, long, conflicts_with_all=&["save_spectrum", "output", "visualization"])]
    pub live_visualization: bool,

    /// Generate spectrum as PNG into target --output
    #[arg(short, long, requires = "output_param", group="save_action", conflicts_with_all=&["live_visualization", "visualization"])]
    pub save_spectrum: bool,

    /// Output png path
    #[arg(short, long, requires = "save_action", group = "output_param", conflicts_with_all=&["live_visualization", "visualization"])]
    pub output: Option<PathBuf>,
}
