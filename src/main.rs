use std::{fs, io, env};




fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut entries = fs::read_dir(args[1].to_string())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    println!("{:#?}", entries);
    // The entries have now been sorted by their path.

    Ok(())
}
