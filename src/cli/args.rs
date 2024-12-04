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
    #[clap(aliases = ["up", "upl", "send", "s"])]
    /// Allows to upload a file using oneshot implementation
    Upload {
        #[arg(num_args(1), required = true)]
        local_path: String,
        #[arg(short, long, aliases = ["rp"])]
        remote_path: Option<String>,
    },
    #[clap(aliases = ["md"])]
    /// Creates directory with given path
    Mkdir {
        #[arg(num_args(1), required = true)]
        path: String,
    },
    #[clap(aliases = ["rd", "mvd"])]
    /// Allows to rename (therefore also move) directories
    RenameDirectory {
        #[arg(num_args(1), required = true)]
        from: String,
        #[arg(num_args(1), required = true)]
        to: String,
    },
    #[clap(aliases = ["get", "dl"])]
    /// Allows to download a file with the given path
    Download {
        #[arg(num_args(1), required = true)]
        path: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    #[clap(aliases = ["dd"])]
    /// Deletes given directory
    DeleteDirectory {
        #[arg(num_args(1), required = true)]
        path: String,
    },
    #[clap(aliases = ["rf", "mv"])]
    /// Allows to rename (therefore also move) files
    RenameFile {
        #[arg(num_args(1), required = true)]
        from: String,
        #[arg(num_args(1), required = true)]
        to: String,
    },
    #[clap(aliases = ["info", "i", "pwd"])]
    /// Displays current bucket info
    BucketInfo {
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["stat"])]
    /// Allows to stat given resource
    StatResource {
        #[arg(num_args(1), required = true)]
        path: String,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["ld"])]
    /// Lists all directories in a bucket
    ListDirectories {
        #[arg(short, long)]
        range: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["lf"])]
    /// Lists all files in a bucket
    ListFiles {
        #[arg(short, long)]
        range: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },
    #[clap(aliases = ["ls"])]
    /// Shows contents of provided directory
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
