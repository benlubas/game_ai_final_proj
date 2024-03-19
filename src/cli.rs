/// Handle the CLI with CLAP

pub mod cli {
    use clap::Parser;

    /// Control RocketLeague to create a game and start different bots.
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        /// Create a new match
        #[arg(short, long)]
        pub start: bool,
    }
}
