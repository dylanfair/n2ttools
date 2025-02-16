use std::path::PathBuf;

use crate::compiler::files::valid_files;
use crate::compiler::parser::Compiler;

pub fn run_compiler(path: String, debug: bool) {
    println!("Running the compiler on '{}'", path);
    let path_buf = PathBuf::from(path);

    let files = valid_files(&path_buf);
    if files.is_none() {
        println!("Could not find any valid '.jack' files to work on.");
        return;
    }

    for file in files.expect("Should have something after .is_none() check") {
        let mut compiler = Compiler::new(file, debug);
        compiler.compile_file();
    }
}
