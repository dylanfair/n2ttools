use std::path::Path;

pub fn parse_vm_file<P>(file: P, debug: bool) -> String
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if debug {
        println!("Parsing {}", file.as_ref().display());
    }

    String::from("Blah")
}
