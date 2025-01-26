use std::path::Path;

pub fn run_vm<P>(file: P, debug: bool)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Running the vm on '{}'", file.as_ref().display());
    if !check_filetype(&file) {
        return;
    }
}

fn check_filetype<P>(file: &P) -> bool
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let filetype = Path::new(file.as_ref()).extension();
    match filetype {
        Some(extension) => {
            if extension != "vm" {
                println!("Path supplied isn't an .vm file");
                return false;
            }
            true
        }
        None => {
            println!("Path supplied isn't an .vm file");
            false
        }
    }
}
