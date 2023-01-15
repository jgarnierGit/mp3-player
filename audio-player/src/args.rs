use std::path::PathBuf;

use clap::ArgAction::Append;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name="J.Garnier", author, version, about, long_about = None)]
pub struct Cli {
    /// Audio path in absolute path
    #[arg(short, long)]
    pub input: PathBuf,
    /// Play audio
    #[arg(short, long)]
    pub play: bool,
    /// Get audio full metadatas & tags
    #[arg(short, long, conflicts_with_all=&["tag"])]
    pub analyze: bool,
    /// Get audio specific tag or metadata i.e ["duration","frameRate","channels","genre"]
    #[arg(short, long, action=Append, conflicts_with_all=&["analyze"])]
    pub tag: Option<Vec<String>>,
    /// Get full audio data
    #[arg(long)]
    pub full_audio_sample: bool,
    /// Get live audio data stream
    #[arg(long, conflicts_with_all=&["play"])]
    pub live_audio_sample: bool,
}
