use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;

const VERSION: &'static str = "9.5-1.1.0";

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let tarball = format!("{}.tar.gz", VERSION);
    let url = format!("https://github.com/lfittl/libpg_query/archive/{}", tarball);
    let build_dir = out_dir.join(format!("libpg_query-{}", VERSION));

    if !out_dir.join(&tarball).exists() {
        run(Command::new("curl").arg("-OL").arg(url).current_dir(&out_dir));
    }

    if !build_dir.exists() {
        run(Command::new("tar").arg("xzf").arg(out_dir.join(tarball)).current_dir(&out_dir));
    }
    let mut command = Command::new("make");
    command.env_remove("PROFILE").arg("-C").arg(&build_dir);
    if env::var("PROFILE").unwrap() == "debug" {
        command.arg("DEBUG=1");
    }
    run(&mut command);

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=pg_query");
}

fn run(command: &mut Command) {
    let status = command.stdin(Stdio::null())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .status()
                        .unwrap();
    assert!(status.success());
}
