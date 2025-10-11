use anyhow::{bail, Result};
use std::env;
use std::io;
use std::path::Path;
use wit_parser::Resolve;

fn main() -> Result<()> {
    let mut expect_error = false;
    let mut paths = Vec::new();
    for arg in env::args().skip(1) {
        if arg == "--expect-error" {
            expect_error = true;
        } else {
            paths.push(arg);
        }
    }

    if paths.len() != 1 {
        bail!("expected exactly one WIT path argument");
    }

    let path = Path::new(&paths[0]);

    let mut resolve = Resolve::new();
    resolve.features.insert("active".to_string());

    let result = resolve.push_path(path);

    match result {
        Ok(_) => {
            if expect_error {
                bail!("expected parsing to fail but it succeeded");
            }
            let json = serde_json::to_string_pretty(&resolve)?;
            println!("{json}");
        }
        Err(mut e) => {
            if !expect_error {
                return Err(e);
            }
            if let Some(err) = e.downcast_mut::<io::Error>() {
                *err = io::Error::new(
                    io::ErrorKind::Other,
                    "some generic platform-agnostic error message",
                );
            }
            println!("{:#}", e);
        }
    }

    Ok(())
}
