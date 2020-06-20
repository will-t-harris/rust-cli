use exitfailure::ExitFailure;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    // try to parse args into Cli struct
    let args = Cli::from_args();
    let path = Path::new(&args.path);
    // read file at path
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }

    Ok(())
}
