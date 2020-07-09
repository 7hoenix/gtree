
use gtree;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let maybe_file = File::open(".gitignore");
    match maybe_file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let ignore_rules: Vec<_> = contents.split("\n").collect();
            println!("rules: {:#?}", ignore_rules);
            let entries = fs::read_dir(".")?;
            gtree::print_gtree(entries, ignore_rules)?;
        },
        Err(_) => {
            println!("no gitignore found!");

            let entries = fs::read_dir(".")?;
            gtree::print_gtree(entries, Vec::new())?;
        }
    };
    Ok(())
}
