use chrono::prelude::{DateTime, Utc};
use colored::*;
use std::{
    env,
    ffi::OsString,
    fs::{self, Metadata},
    io,
};

#[derive(Debug)]
struct DirInfo {
    name: OsString,
    is_dir: bool,
    file_ext: Option<OsString>,
    metadata: io::Result<Metadata>,
}

impl DirInfo {
    fn format(&self) -> String {
        let metadata = self.metadata.as_ref().unwrap();

        let can_write: ColoredString = match !metadata.permissions().readonly() {
            true => "true".bright_green(),
            false => "false".bright_red(),
        };

        let size = metadata.len();

        let modify: DateTime<Utc> = metadata.modified().unwrap().into();
        let edited = modify.format("%d/%m/%Y %H:%M");

        let create: DateTime<Utc> = metadata.created().unwrap().into();
        let created = create.format("%d/%m/%Y %H:%M");

        let file_ext: OsString = if self.file_ext.is_none() {
           self.name.to_owned()
        } else {
           self.file_ext.as_ref().unwrap().to_os_string()
        };

        let icon = match file_ext.to_str().unwrap() {
            "md" => "".bright_blue(),
            ".gitignore" | ".git" => "".bright_red(),
            "rs" => "".red(),
            ".github" => "".white(),
            "src" => "".white(),
            "exe" | "o" => "".white(),
            "toml" => "".red(),
            "lock" => "".yellow(),
            "js" => "".yellow(),
            "cpp" => "".blue(),
            "h" => "H".blue(),
            "c" => "".blue(),
            "rain" => "".bright_blue(),
            _ => " ".white(),
        };
        let dir_icon = match self.is_dir {
            true => "",
            false => "",
        };

        format!(
            "{} {} {}{}\n\t-- Last edited: {}\n\t-- Can write: {}\n\t-- Created: {}\n",
            icon,
            self.name.to_str().unwrap(),
            dir_icon,
            format!("\n\t-- 💾 Size (bytes): {size}"),
            edited,
            can_write,
            created
        )
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut dir = "./";

    if args.len() > 1 {
        dir = &args[1];
    }

    println!("Dir: {}", dir.to_string().bright_green().bold());

    // let mut entries = fs::read_dir(dir)?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    let entries = fs::read_dir(dir)?
        .map(|res| {
            res.map(|e| DirInfo {
                name: e.file_name(),
                is_dir: !e.metadata().unwrap().is_file(),
                file_ext: e.path().extension().map(|s| s.to_owned()),
                metadata: e.metadata(),
            })
        })
        .collect::<Result<Vec<DirInfo>, io::Error>>();

    match entries {
        Ok(ent) => {
            for file in ent {
                println!("{}", file.format());
            }
        }
        Err(x) => panic!("{}", x),
    }

    Ok(())
}
