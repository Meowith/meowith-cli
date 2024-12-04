use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser, Clone)]
#[command(name = "meowith")]
#[clap(aliases = ["mcli"])]
/// Meowith Command line interface tool
pub struct MeowithCommand {
    #[command(subcommand)]
    pub command: MeowithSubCommand,
}

#[derive(Subcommand, Clone)]
pub enum MeowithSubCommand {
    #[clap(aliases = ["a"])]
    /// Allows to set token for future operations
    Auth {
        #[arg(num_args(1), required = true)]
        token: String,
    },
    #[clap(aliases = ["u"])]
    /// Allows to set app and bucket id for future operations
    Use {
        #[arg(num_args(1), required = true)]
        app_id: Uuid,
        #[arg(num_args(1), required = true)]
        bucket_id: Uuid,
        #[arg(num_args(1), required = true)]
        node: String,
    },
    #[clap(aliases = ["stat"])]
    StatResource {
        #[arg(num_args(1), required = true)]
        path: String,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["ld"])]
    ListDirectories {
        #[arg(short, long)]
        range: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["lf"])]
    ListFiles {
        #[arg(short, long)]
        range: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["ls"])]
    Directory {
        #[arg(num_args(1), required = true)]
        path: String,
        #[arg(short, long)]
        range: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["v"])]
    /// Displays version information
    Version,
}
