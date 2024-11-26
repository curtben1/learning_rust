use std::fs;
use std::path::Path;

fn delete_everything_in_parent() -> std::io::Result<()> {
    let parent_dir = Path::new("..");
    for entry in fs::read_dir(parent_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn main() {
    println!("Deleting everything in the parent directory...");
    match delete_everything_in_parent() {
        Ok(_) => println!("Successfully deleted everything in the parent directory."),
        Err(e) => eprintln!("Error deleting files: {}", e),
    }
}
