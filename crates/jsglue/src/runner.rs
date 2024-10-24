use std::{
    ffi::OsStr,
    process::{ExitStatus, Stdio},
};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

use crate::framework::Framework;

pub static mut CLIENT_READY: bool = false;

pub fn read_lines(child: &mut Child, framework: Framework) {
    let stdout = child
        .stdout
        .take()
        .expect("Child did not have a handle to stdout!");

    let stderr = child
        .stderr
        .take()
        .expect("Child did not have a handle to stderr!");

    let mut stdout = BufReader::new(stdout).lines();
    let mut stderr = BufReader::new(stderr).lines();

    tokio::spawn(async move {
        while let Ok(Some(line)) = stdout.next_line().await {
            unsafe {
                if CLIENT_READY && !line.trim().is_empty() {
                    framework.process_message(&line);
                }
            }

            if line.trim().contains(framework.get_ready_str()) {
                info!("Started client!");

                unsafe {
                    CLIENT_READY = true;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Ok(Some(line)) = stderr.next_line().await {
            if !line.trim().is_empty() {
                framework.process_message(&line);
            }
        }
    });
}

pub async fn start_client<T>(dir: T, mut cmd: Vec<T>, framework: Framework) -> ExitStatus
where
    T: AsRef<OsStr>,
{
    cmd.reverse();

    let exec = cmd.pop().unwrap();

    cmd.reverse();

    let mut cmd = Command::new(exec.as_ref())
        .args(cmd)
        .current_dir(dir.as_ref())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    read_lines(&mut cmd, framework);

    cmd.wait().await.unwrap()
}
