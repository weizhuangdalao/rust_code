use clap::Parser;
use rcli::{Opts, Subcommand, process_csv};
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
    };
    Ok(())
}