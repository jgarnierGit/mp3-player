use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name="J.Garnier", author, version, about, long_about = None)]
pub struct Cli {
    /// Audio folder in absolute path to parse recursively
    #[arg(short, long, required = true)]
    pub input: PathBuf,

    /// Count audio files
    #[arg(short, long, conflicts_with_all=&["aggregate_tag", "filter_tag", "filter_value"])]
    pub count: bool,

    /// Aggregates and count audio files by given tag i.e ["frameRate","channels","genre"]
    #[arg(short, long,  conflicts_with_all=&["filter_tag", "count", "filter_value"])]
    pub aggregate_tag: Option<String>,

    /// Filter value to apply if --filter-tag (-f) is set
    #[arg(long, requires = "filter_val", group = "filter", conflicts_with_all=&["aggregate_tag", "count"])]
    pub filter_value: Option<String>,

    /// Filters audio files by given tag i.e ["frameRate","channels","genre"] and value given by --filter_value.
    /// May take several minutes to process.
    /// Is case sensitive
    #[arg(short, long, group = "filter_val", requires = "filter", conflicts_with_all=&["aggregate_tag", "count"])]
    pub filter_tag: Option<String>,
}
