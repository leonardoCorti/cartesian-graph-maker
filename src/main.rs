use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "synonymous_plotter")]
#[command(about = "A program to create a graph from a database synonymous", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// server
    Serve {},
    /// graph operations
    Graph {},
    /// database operations
    Db {
        #[command(subcommand)]
        action: Option<DbCommand>,
    },
}

#[derive(Subcommand)]
enum DbCommand {
    Populate {},
    Query {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command.unwrap_or(Commands::Serve {}) {
        Commands::Serve {} => {
            println!("server");
        }
        Commands::Graph {} => {
            println!("graph operations");
        }
        Commands::Db { action: _ } => {
            println!("Database operations");
        }
    }
}
