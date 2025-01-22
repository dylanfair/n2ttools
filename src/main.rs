use clap::{Parser, Subcommand};

mod assembler;

use assembler::run::run_assembler;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Assembler { file, debug }) => {
            run_assembler(file, debug);
        }
        Some(Commands::Compiler { file }) => {
            todo!();
        }
        None => {}
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs the assembler on your file of choice
    Assembler {
        #[arg()]
        file: String,

        #[arg(long)]
        debug: bool,
    },
    Compiler {
        #[arg()]
        file: String,
    },
}
