use ructe::{Result, Ructe};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("modules");
    ructe.compile_templates(in_dir)
}
