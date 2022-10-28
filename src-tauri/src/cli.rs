use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(version = "0.0.1", about = "App to test CLAP features.")]
pub struct Arguments {
    #[clap(short, long, help = "Displays debug information.")]
    pub debug: bool,
    #[clap(
        short,
        long,
        help = "Log messages to a file. Defaults to the default log file. Automatically enables debugging."
    )]
    pub log: bool,
    #[clap(long, help = "Sets the log file. Automatically enables logging.")]
    pub log_file: Option<String>,
}