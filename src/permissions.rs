use std::{fs, io, os::unix::fs::PermissionsExt, process::Command};

#[derive(Debug)]
enum CheckError {
  BinaryNotFound(String),
  IoError(io::Error),
  NonZeroExit(i32),
}

impl std::fmt::Display for CheckError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      CheckError::BinaryNotFound(bin) => write!(f, "binary not found: {bin}"),
      CheckError::IoError(e) => write!(f, "io error: {e}"),
      CheckError::NonZeroExit(code) => write!(f, "exited with code {code}"),
    }
  }
}

fn run(cmd: &str, args: &[&str]) -> Result<std::process::Output, CheckError> {
  match Command::new(cmd).args(args).output() {
    Ok(out) => Ok(out),
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
      Err(CheckError::BinaryNotFound(cmd.to_string()))
    }
    Err(e) => Err(CheckError::IoError(e)),
  }
}

/// Check if current user is in a given group
fn in_group(group_name: &str) -> Result<bool, CheckError> {
  let out = run("id", &["-Gn"])?;
  if !out.status.success() {
    return Err(CheckError::NonZeroExit(out.status.code().unwrap_or(-1)));
  }
  Ok(
    String::from_utf8_lossy(&out.stdout)
      .split_whitespace()
      .any(|g| g == group_name),
  )
}

/// Check if running as root (uid 0)
fn is_root() -> Result<bool, CheckError> {
  let out = run("id", &["-u"])?;
  if !out.status.success() {
    return Err(CheckError::NonZeroExit(out.status.code().unwrap_or(-1)));
  }
  Ok(String::from_utf8_lossy(&out.stdout).trim() == "0")
}

/// Prime sudo cache once. Ok(true) if authenticated, Ok(false) if auth failed or denied
fn ensure_sudo() -> Result<bool, CheckError> {
  let status = match Command::new("sudo").arg("-v").status() {
    Ok(s) => s,
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
      return Err(CheckError::BinaryNotFound("sudo".to_string()));
    }
    Err(e) => return Err(CheckError::IoError(e)),
  };
  Ok(status.success())
}

/// Check read/write/execute access on a path (real uid, via `test`)
fn can_access(path: &str, mode: &str) -> Result<bool, CheckError> {
  let status = match Command::new("test").arg(mode).arg(path).status() {
    Ok(s) => s,
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
      return Err(CheckError::BinaryNotFound("test".to_string()));
    }
    Err(e) => return Err(CheckError::IoError(e)),
  };
  Ok(status.success())
}

/// Raw permission bits, pure std, distinguishes not found / permission denied / other
fn perm_bits(path: &str) -> io::Result<u32> {
  fs::metadata(path).map(|m| m.permissions().mode())
}
