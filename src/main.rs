use anyhow::format_err;
use std::fs::read_dir;
use std::fs::rename;
use std::fs::ReadDir;

const PREFIX: &str = "b-";

// 1. implement prefix removal
// 2. make the prefix a command line argument
// 3. make the renaming recursive (for nested file)

fn prefix_addition(fp: ReadDir) -> anyhow::Result<()> {
    println!("========= PREFIX ADDITION =========");
    for file in fp {
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
    println!("");
    Ok(())
}

fn prefix_removal(fp: ReadDir) -> anyhow::Result<()> {
    println!("========= PREFIX REMOVAL =========");
    for file in fp {
        let mut file_path = file?.path();
        let old_file_path = file_path.clone();
        let file_name = file_path
            .file_name()
            .ok_or_else(|| format_err!("failed to get file name"))?
            .to_str()
            .ok_or_else(|| format_err!("failed to convert to str"))?
            .strip_prefix(PREFIX)
            .ok_or_else(|| format_err!("failed to find the prefix"))?
            .to_string();

        file_path.set_file_name(file_name);
        rename(old_file_path, file_path)?;
    }
    println!("Successfully removed prefixes!\n");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let files = read_dir("./")?;

    //// implement prefix addition
    prefix_addition(files)?;

    //// implement prefix removal
    // prefix_removal(files)?;

    Ok(())
}
