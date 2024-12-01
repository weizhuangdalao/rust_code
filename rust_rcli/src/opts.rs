use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[command(name = "csv", about = "show csv, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = '|')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if std::path::Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("文件找不到！！".into())
    }
}