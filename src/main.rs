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
    /// Takes in a .asm file and returns a .hack file
    Assembler {
        #[arg()]
        file: String,

        #[arg(long)]
        debug: bool,
    },
    /// Takes in a .vm file and returns a .asm file
    Vm {
        #[arg(default_value = ".")]
        file: String,

        #[arg(long)]
        debug: bool,
    },
}
