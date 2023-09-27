use std::{fs, io, env};
use colored::*;



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut dir = args[1].to_string();

    if args.len() < 1 {
        dir = String::from("./");
    }

    if args[2] == "help" {
        println!(
            "{} - Nikita\'s version of ls\n{}", "rd".bright_blue(), "Usage: rd [path]".blue()
        );
        std::process::exit(0);
    }

    let path = env::current_dir()?;
    println!("Current dir {}", path.display());

    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    for i in 0..entries.len() {
        println!("{}: {}", i.to_string(), String::from(entries[i].to_str().unwrap()).blue());
    }
    // The entries have now been sorted by their path.

    Ok(())
}
