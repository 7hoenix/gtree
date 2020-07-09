use std::fs;
use gitignore;

pub fn print_gtree(entries: fs::ReadDir, gitignore: Option<gitignore::File>) -> std::io::Result<()> {
    for entry in entries {
        let dir = entry?;
        let mut ignore = false;

        if let Some(g) = &gitignore {
            match g.is_excluded(&dir.path()) {
                Ok(true) => {
                    ignore = true;
                },
                Ok(false) => {
                },
                Err(e) => {
                    println!("failing:{:?}", e);
                }
            }
        }
        if !ignore {
        } else {
            println!("{:?}", dir.file_name());
        }
    }
    Ok(())
}
