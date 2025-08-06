use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "cartesian_graph_maker")]
#[command(about = "A program to create a cartesian graph from a database", long_about = None)]
struct Cli {
    /// database location
    #[arg(short, long, global = true, default_value = "./data.db")]
    db: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// server
    Serve {
        #[arg(long, default_value = "0.0.0.0")]
        address: String,
        #[arg(long, default_value = "8091")]
        port: String,
    },
    /// graph operations
    Graph {
        #[arg(value_enum)]
        format: ImageFormat,
        #[arg(short, long)]
        x_attribute: Option<String>,
        #[arg(short, long)]
        y_attribute: Option<String>,
        #[arg(short, long, default_value = "1080")]
        height: usize,
    },
    /// database operations
    Db {
        #[command(subcommand)]
        action: Option<DbCommand>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ImageFormat {
    Png,
    Svg,
}

#[derive(Subcommand)]
enum DbCommand {
    Populate {},
    Query {},
    Create {},
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.db);

    match &cli.command.unwrap_or(Commands::Serve {
        address: "0.0.0.0".to_string(),
        port: "8091".to_string(),
    }) {
        Commands::Serve {
            address: _,
            port: _,
        } => {
            println!("server");
        }
        Commands::Graph {
            format: _,
            x_attribute: _,
            y_attribute: _,
            height: _,
        } => {
            println!("graph operations");
        }
        Commands::Db { action: _ } => {
            println!("Database operations");
        }
    }
}
