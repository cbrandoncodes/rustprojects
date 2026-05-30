use std::error::Error;
use std::process;

use mini_grep::{Config, run};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    let output = run(config)?;

    if !output.is_empty() {
        println!("{output}");
    }

    Ok(())
}
