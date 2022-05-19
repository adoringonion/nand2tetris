use std::{env, path::Path};

use anyhow::{anyhow, Context, Result};

mod jack_tokenizer;
mod compilation_engine;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        return Err(anyhow!("Invalid args."))
    }

    let file_path = Path::new(&args[0]);
    match file_path.extension() {
        Some(extension) => {
            if extension != "jack" {
                return Err(anyhow!("Invalid extension file. You can load only jack file"));
            }

            
        },
        None => {
            return Err(anyhow!("Invalid extension file. You can load only jack file"));
        }
    }

    return Ok(());
}
