use clap::{Parser, Subcommand};

mod assembler;
mod vm;

use assembler::run::run_assembler;
use vm::run::run_vm;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Assembler { file, debug }) => {
            run_assembler(file, debug);
        }
        Some(Commands::Vm { file, debug }) => {
            run_vm(file, debug);
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
    Vm {
        #[arg()]
        file: String,

        #[arg(long)]
        debug: bool,
    },
}
