use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

mod progress_bar;

#[test]
fn check_43(){
    assert_eq!(progress_bar::run(), 43);
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    //let path = &args.path;
    let path = "test.txt";
    let content =
        std::fs::read_to_string(path).with_context(|_| format!("Could not read file {}", path))?;
    println!("file content: {}", content);
    return Ok(());
}

