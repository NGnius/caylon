use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to config file
    #[clap(long)]
    pub config: Option<std::path::PathBuf>,

    /// Path to log file
    #[clap(long)]
    pub log: Option<std::path::PathBuf>,
}

impl CliArgs {
    pub fn cli() -> Self {
        Self::parse()
    }
}
