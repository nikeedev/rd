use chrono::prelude::{DateTime, Utc};
use colored::*;
use std::{
    env,
    ffi::OsString,
    fs::{self, Metadata},
    io
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

        let size = format!("{}", metadata.len() as f32 / 1000.0);

        let modify: DateTime<Utc> = metadata.modified().unwrap().into();
        let edited = format!("{}", modify.format("%d/%m/%Y %H:%M"));

        let file_ext: OsString = if self.file_ext.is_none() {
           self.name.to_owned()
        } else {
           self.file_ext.as_ref().unwrap().to_os_string()
        };

        let icon = match file_ext.to_str().unwrap() {
            "md" => "".bright_blue(),
            ".gitignore" | ".git" => "".bright_red(),
            "rs" | "rust" => "".red(),
            ".github" => "".bright_white(),
            "src" => "".bright_white(),
            "exe" | "o" => "".bright_white(),
            "toml" => "".red(),
            "lock" => "".yellow(),
            "js" | "javascript" => "".yellow(),
            "cpp" => "".blue(),
            "h" => "H".blue(),
            "c" => "".blue(),
            "rain" => "".bright_blue(),
            "html" => "".bright_red(),
            "txt" => "".bright_white(),
            "css" => "".bright_blue(),
            "LICENSE" => "".red(),
            "CNAME" => "".red(),
            _ => " ".bright_white(),
        };
        let dir_icon = if self.is_dir {
            ""
        } else {
            ""
        };

        let size_text = if !self.is_dir {
            format!("{} ", size.clone())
        } else {
            "      ".to_string()
        };
     
        let num = if (size_text.len()-4) > 2 || (size_text.len()-4) < 2 {
            size_text.len()-6
        } else {
            0
        };

        format!(
            "{}{}{}{}{} {} {}",
            edited.blue(),
            " ".repeat(10-(size_text.len()-4)),
            size_text,
            " ".repeat(size_text.len()-num),
            icon,
            self.name.to_str().unwrap(),
            dir_icon,
        )
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut dir = "./";

    if args.len() > 1 {
        dir = &args[1];
    }

    let current_dir = env::current_dir().unwrap();
    println!("Current dir: {}\nDir: {}", current_dir.display(), dir.to_string().bright_green().bold());

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
    
    println!("Last edited           💾 Size (KB)   Name");
    println!("-----------           ------------   ---------");
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
