use anyhow::format_err;
use std::fs::read_dir;
use std::fs::rename;

const PREFIX: &str = "b-";

// 1. implement prefix removal
// 2. make the prefix a command line argument
// 3. make the renaming recursive (for nested file)

fn main() -> anyhow::Result<()> {
    let files = read_dir("./")?;
    for file in files {
        let mut file_path = file?.path();
        let old_file_path = file_path.clone();
        let file_name = file_path
            .file_name()
            .ok_or_else(|| format_err!("failed to get file name"))?
            .to_str()
            .ok_or_else(|| format_err!("failed to convert to str"))?;
        println!("Before file name: {}", file_name);

        file_path.set_file_name(PREFIX.to_owned() + file_name);
        println!(
            "After file name: {}",
            file_path
                .file_name()
                .ok_or_else(|| format_err!("failed to get file name"))?
                .to_str()
                .ok_or_else(|| format_err!("failed to convert to str"))?
        );

        rename(old_file_path, file_path)?; // std::result::Result<(), std::io::Error>
    }

    Ok(())
}
