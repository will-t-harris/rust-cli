use structopt::StructOpt;

// Search for a pattern in a file & display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // read file at path
    let content = std::fs::read_to_string(&args.path)?;

    println!("file content: {}", content);
    Ok(())
}
