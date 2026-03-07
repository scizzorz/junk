use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(ClapParser)]
#[command(about = "Convert .junk files to JSON")]
struct Cli {
    /// Input .junk files
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Output directory for generated .json files
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut failed = false;

    for path in &cli.files {
        let contents = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading {}: {e}", path.display());
                failed = true;
                continue;
            }
        };

        let json = match junk::parse_junk(&contents) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("{}: {err}", path.display());
                failed = true;
                continue;
            }
        };

        let stem = path.file_stem().unwrap_or_default();
        let out_path = cli.output.join(stem).with_extension("json");
        if let Err(e) = fs::write(&out_path, json) {
            eprintln!("error writing {}: {e}", out_path.display());
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
