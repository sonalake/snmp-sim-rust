use std::ffi::OsString;

// generated by `sqlx migrate build-script`
fn main() -> std::io::Result<()> {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-env-changed=MAN_DIR");

    let package_name: &str = env!("CARGO_PKG_NAME").into();
    let man_dir = std::path::PathBuf::from(std::env::var_os("MAN_DIR").map_or(OsString::from("./man"), |v| v));

    let cmd = clap::Command::new(package_name)
        .arg(clap::arg!(-n --name <NAME>))
        .arg(clap::arg!(-c --count <NUM>));

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_dir.join(format!("{}.1", package_name)), buffer)?;

    Ok(())
}
