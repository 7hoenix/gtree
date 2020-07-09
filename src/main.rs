
use gtree;
use std::fs;
use std::fs::File;
use gitignore;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let maybe_file = File::open(".gitignore");
    match maybe_file {
        Ok(_) => {
            let gitignore_path = Path::new(".gitignore");
            let gitignore_from_lib = gitignore::File::new(gitignore_path).unwrap();

            let entries = fs::read_dir(".")?;
            gtree::print_gtree(entries, Some(gitignore_from_lib))?;
        },
        Err(_) => {
            println!("no gitignore found!");

            let entries = fs::read_dir(".")?;
            gtree::print_gtree(entries, None)?;
        }
    };
    Ok(())
}
