use colored::*;
use std::{env, fs, io, ffi::OsString};

mod file_types;


#[derive(Debug)]
struct DirInfo {
    name: OsString,
    is_dir: bool,
    file_ext: Option<OsString>
}

impl DirInfo {
    fn print(&self) {
        
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut dir = "./";
    
    if args.len() > 2 {
        if args[2] == "--help" {
            println!(
                "{} - Nikita\'s version of ls\n{}",
                "rd".bright_blue(),
                "Usage: rd [path]".blue()
            );
            std::process::exit(0);

        }
        dir = &args[1];
    }

    let path = env::current_dir()?.display().to_string();
    println!("{}", format!("Current dir  : {path}").bright_green().bold());

    // let mut entries = fs::read_dir(dir)?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    let entries = fs::read_dir(dir)?
        .map(|res| {
            res.map(|e| {
                DirInfo { name: e.file_name(), is_dir: !e.metadata().unwrap().is_file(), file_ext: e.path().extension().map(|s| s.to_owned()) }
            })
        }).collect::<Result<Vec<DirInfo>, io::Error>>()?;

    println!("{:#?}", entries);

    Ok(())
}
