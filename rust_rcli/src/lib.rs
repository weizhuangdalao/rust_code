mod opts;
mod process;

pub use opts::{Opts, OutputFormat, Subcommand};
pub use process::{process_csv, process_genpass};