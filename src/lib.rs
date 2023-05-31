use std::{
    io::{self, Error, ErrorKind},
    process::Command,
};

#[derive(Debug, Clone)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
}

/// Runs shell command(s) and returns the output.
#[allow(dead_code)]
pub fn run(cmd: &str) -> io::Result<Output> {
    log::debug!("running '{cmd}'");
    match Command::new("sh").args(&["-c", cmd]).output() {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).as_ref().to_owned();
            let stderr = String::from_utf8_lossy(&o.stderr).as_ref().to_owned();
            if o.status.success() {
                Ok(Output { stdout, stderr })
            } else {
                match o.status.code() {
                    Some(code) => {
                        log::warn!("command failed with status code {}: {}", code, stderr);
                        Err(Error::new(
                            ErrorKind::Other,
                            format!("command failed with status code {}: {}", code, stderr),
                        ))
                    }
                    None => {
                        log::warn!(
                            "command terminated by signal with no status code: {}",
                            stderr
                        );
                        Err(Error::new(
                            ErrorKind::Other,
                            format!(
                                "command terminated by signal with no status code: {}",
                                stderr
                            ),
                        ))
                    }
                }
            }
        }
        Err(e) => {
            log::warn!("command failed: {}", e);
            Err(e)
        }
    }
}

#[test]
fn test_bash_run() {
    let output = run("ls -lah .").unwrap();
    println!("stdout:\n{}", output.stdout);
    println!("stderr:\n{}", output.stderr);
}
