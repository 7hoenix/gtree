use std::fs;

pub fn print_gtree(entries: fs::ReadDir, ignore_rules: Vec<&str>) -> std::io::Result<()> {
    for entry in entries {
        let dir = entry?;
        for rule in &ignore_rules {
            if let Some(file_name) = dir.file_name().to_str() {
                if &file_name == rule {
                } else {
                    println!("{:?}", dir.file_name());
                }
            };
        }
    }
    Ok(())
}