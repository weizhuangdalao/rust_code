use clap::Parser;
use rcli::{Opts, Subcommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?
        }
    };
    Ok(())
}