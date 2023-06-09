use anyhow::format_err;
use clap::Parser;
use std::fs::read_dir;
use std::fs::rename;
use std::fs::ReadDir;

// const PREFIX: &str = "b-";

// 1. implement prefix removal
// 2. make the prefix a command line argument
// 3. make the renaming recursive (for nested file)

fn prefix_addition(fp: ReadDir, prefix: &str) -> anyhow::Result<()> {
    println!("========= PREFIX ADDITION =========");
    for file in fp {
        // validate file
        let entry = file?;
        if !entry.path().as_path().is_file() {
            continue;
        }

        let mut file_path = entry.path();
        let old_file_path = file_path.clone();
        let file_name = file_path
            .file_name()
            .ok_or_else(|| format_err!("failed to get file name"))?
            .to_str()
            .ok_or_else(|| format_err!("failed to convert to str"))?;
        println!("Before file name: {}", file_name);

        file_path.set_file_name(prefix.to_owned() + file_name);
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

fn prefix_removal(fp: ReadDir, prefix: &str) -> anyhow::Result<()> {
    println!("========= PREFIX REMOVAL =========");
    let mut is_removed: bool = false;
    for file in fp {
        let mut file_path = file?.path();
        let old_file_path = file_path.clone();
        let does_prefix_exist = file_path
            .file_name()
            .ok_or_else(|| format_err!("failed to get file name"))?
            .to_str()
            .ok_or_else(|| format_err!("failed to convert to str"))?
            .strip_prefix(prefix);

        if !does_prefix_exist.is_none() {
            //// TODO: refactor .unwrap
            let file_name = does_prefix_exist.unwrap().to_string();
            file_path.set_file_name(file_name);
            rename(old_file_path, file_path)?;
            is_removed = true;
        }
    }
    if is_removed {
        println!("Successfully removed prefixes!\n");
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    add: bool,

    #[arg(short, long)]
    prefix: String,
}

fn main() -> anyhow::Result<()> {
    let files = read_dir("./")?;

    //// implement CLI
    let args = Cli::parse();

    if args.add {
        //// implement prefix addition
        prefix_addition(files, &args.prefix)?;
    } else {
        //// implement prefix removal
        prefix_removal(files, &args.prefix)?;
    }

    Ok(())
}
