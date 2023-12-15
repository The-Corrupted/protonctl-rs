use clap_complete::generate_to;
use clap_complete_nushell::Nushell;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(out) => out,
    };

    let mut cmd = build_cli();
    let path = generate_to(Nushell, &mut cmd, "protonctl", outdir)?;
    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
