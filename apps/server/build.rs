use std::env::consts::OS;

#[cfg(not(debug_assertions))]
fn build() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("ui");

    std::process::Command::new("bun")
        .arg("install")
        .current_dir(path.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::process::Command::new("bun")
        .arg("run")
        .arg("build")
        .current_dir(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[cfg(debug_assertions)]
fn build() {
    println!("Not running the frontend's build script, this is a debug build.");
}

fn main() {
    if OS == "windows" {
        return;
    }

    build();
}
