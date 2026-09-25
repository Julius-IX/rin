use std::{
  fmt, fs, io,
  io::Write,
  process::{Command, Output, Stdio},
};

#[derive(Debug)]
pub enum CheckError {
  BinaryNotFound(String),
  IoError(io::Error),
  NonZeroExit { program: String, code: i32 },
}

impl fmt::Display for CheckError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CheckError::BinaryNotFound(bin) => write!(
        f,
        "`{bin}` isn't installed (or not on PATH) - install it via corresponding package manager"
      ),
      CheckError::IoError(e) => write!(f, "io error: {e}"),
      CheckError::NonZeroExit { program, code } => {
        write!(f, "`{program}` exited with code {code}")
      }
    }
  }
}

impl std::error::Error for CheckError {}

impl From<CheckError> for io::Error {
  fn from(e: CheckError) -> Self {
    io::Error::new(io::ErrorKind::Other, e.to_string())
  }
}

fn run(cmd: &str, args: &[&str]) -> Result<Output, CheckError> {
  Command::new(cmd)
    .args(args)
    .output()
    .map_err(|e| match e.kind() {
      io::ErrorKind::NotFound => CheckError::BinaryNotFound(cmd.to_string()),
      _ => CheckError::IoError(e),
    })
}

fn run_ok(cmd: &str, args: &[&str]) -> Result<String, CheckError> {
  let out = run(cmd, args)?;
  if !out.status.success() {
    return Err(CheckError::NonZeroExit {
      program: cmd.to_string(),
      code: out.status.code().unwrap_or(-1),
    });
  }
  Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Current user's login name.
pub fn current_user() -> Result<String, CheckError> {
  run_ok("id", &["-un"])
}

/// Is the current user root (uid 0)?
pub fn is_root() -> Result<bool, CheckError> {
  Ok(run_ok("id", &["-u"])? == "0")
}

/// Is the current user in `group_name`?
pub fn in_group(group_name: &str) -> Result<bool, CheckError> {
  Ok(
    run_ok("id", &["-Gn"])?
      .split_whitespace()
      .any(|g| g == group_name),
  )
}

/// Does this group exist on the system at all?
pub fn group_exists(group_name: &str) -> Result<bool, CheckError> {
  Ok(run("getent", &["group", group_name])?.status.success())
}

/// Prime the sudo timestamp cache (prompts for a password if needed).
/// Ok(true) if authenticated, Ok(false) if the user declined/failed auth.
pub fn ensure_sudo() -> Result<bool, CheckError> {
  match Command::new("sudo").arg("-v").status() {
    Ok(s) => Ok(s.success()),
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
      Err(CheckError::BinaryNotFound("sudo".to_string()))
    }
    Err(e) => Err(CheckError::IoError(e)),
  }
}

/// Check read/write/execute access on a path for the real uid (`test -r/-w/-x`).
pub fn can_access(path: &str, mode: &str) -> Result<bool, CheckError> {
  match Command::new("test").arg(mode).arg(path).status() {
    Ok(s) => Ok(s.success()),
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
      Err(CheckError::BinaryNotFound("test".to_string()))
    }
    Err(e) => Err(CheckError::IoError(e)),
  }
}

/// Raw permission bits, pure std, no subprocess.
pub fn perm_bits(path: &str) -> io::Result<u32> {
  use std::os::unix::fs::PermissionsExt;
  fs::metadata(path).map(|m| m.permissions().mode())
}

/// The group that owns a device file, or `None` if the path doesn't exist.
pub fn owning_group(path: &str) -> Result<Option<String>, CheckError> {
  if fs::metadata(path).is_err() {
    return Ok(None);
  }
  run_ok("stat", &["-c", "%G", path]).map(Some)
}

/// First entry in `dir` whose name starts with `prefix`
fn first_matching(dir: &str, prefix: &str) -> Option<String> {
  let mut names: Vec<String> = fs::read_dir(dir)
    .ok()?
    .filter_map(|e| e.ok())
    .filter_map(|e| e.file_name().into_string().ok())
    .filter(|n| n.starts_with(prefix))
    .collect();
  names.sort();
  names.into_iter().next().map(|n| format!("{dir}/{n}"))
}

#[derive(Clone, Copy)]
pub enum Target {
  Uinput,
  Input,
}

impl Target {
  fn label(self) -> &'static str {
    match self {
      Target::Uinput => "uinput (creating virtual input devices)",
      Target::Input => "input (reading from /dev/input/event*)",
    }
  }

  fn default_group(self) -> &'static str {
    match self {
      Target::Uinput => "uinput",
      Target::Input => "input",
    }
  }

  fn sample_device(self) -> Option<String> {
    match self {
      Target::Uinput => fs::metadata("/dev/uinput")
        .ok()
        .map(|_| "/dev/uinput".to_string()),
      Target::Input => first_matching("/dev/input", "event"),
    }
  }

  fn udev_rule_filename(self) -> &'static str {
    match self {
      Target::Uinput => "/etc/udev/rules.d/99-rin-uinput.rules",
      Target::Input => "/etc/udev/rules.d/99-rin-input.rules",
    }
  }

  fn udev_rule_body(self, group: &str) -> String {
    match self {
      Target::Uinput => format!(
        "KERNEL==\"uinput\", MODE=\"0660\", GROUP=\"{group}\", OPTIONS+=\"static_node=uinput\"\n"
      ),
      Target::Input => format!("SUBSYSTEM==\"input\", GROUP=\"{group}\", MODE=\"0660\"\n"),
    }
  }

  fn help_url(self) -> &'static str {
    match self {
      Target::Uinput => {
        "https://wiki.archlinux.org/title/Udev#Allowing_regular_users_to_use_uinput"
      }
      Target::Input => {
        "https://wiki.archlinux.org/title/Udev#Allowing_regular_users_to_use_a_device_as_normal_user"
      }
    }
  }
}

pub struct AccessInfo {
  pub target: Target,
  pub device: Option<String>,
  /// Group actually governing the device right now, if any (never "root").
  pub group: Option<String>,
  pub group_exists: bool,
  pub user_in_group: bool,
  pub is_root: bool,
  pub readable: bool,
  pub writable: bool,
}

pub fn inspect(target: Target) -> Result<AccessInfo, CheckError> {
  let root = is_root()?;
  let device = target.sample_device();
  let device_group = match &device {
    Some(d) => owning_group(d)?.filter(|g| g != "root"),
    None => None,
  };
  let group_name = device_group
    .clone()
    .unwrap_or_else(|| target.default_group().to_string());
  let group_registered = group_exists(&group_name)?;
  let user_in_group = if group_registered {
    in_group(&group_name)?
  } else {
    false
  };
  let (readable, writable) = match &device {
    Some(d) => (can_access(d, "-r")?, can_access(d, "-w")?),
    None => (false, false),
  };
  Ok(AccessInfo {
    target,
    device,
    group: device_group,
    group_exists: group_registered,
    user_in_group,
    is_root: root,
    readable,
    writable,
  })
}

fn yn(b: bool) -> &'static str {
  if b { "yes" } else { "no" }
}

pub fn print_status() {
  println!("user: {}", current_user().unwrap_or_else(|_| "?".into()));
  for target in [Target::Uinput, Target::Input] {
    println!();
    println!(
      "== {} ==",
      match target {
        Target::Uinput => "uinput",
        Target::Input => "input",
      }
    );
    match inspect(target) {
      Ok(info) => print_access_info(&info),
      Err(e) => println!("  couldn't check: {e}"),
    }
  }
}

fn print_access_info(info: &AccessInfo) {
  if info.is_root {
    println!("  running as root - full access regardless of group membership");
    return;
  }
  match &info.device {
    Some(d) => println!("  device: {d}"),
    None => println!("  device: not present (module not loaded / no devices yet)"),
  }
  match &info.group {
    Some(g) => println!(
      "  governing group: {g} (access: {})",
      yn(info.user_in_group)
    ),
    None => println!(
      "  governing group: none yet - device isn't gated by a group (default would be `{}`, exists: {})",
      info.target.default_group(),
      yn(info.group_exists)
    ),
  }
  println!(
    "  read access: {}\n  write access: {}",
    yn(info.readable),
    yn(info.writable)
  );
  if info.readable && info.writable {
    println!("  \u{2713} you already have access");
  } else {
    let flag = match info.target {
      Target::Uinput => "--uinput",
      Target::Input => "--input",
    };
    println!(
      "  \u{2717} missing access - run `rin perm {flag}` to fix ({})",
      info.target.help_url()
    );
  }
}

struct PlannedCommand {
  explain: String,
  argv: Vec<String>,
  /// Content to pipe to stdin, used for writing files via `sudo tee`.
  stdin: Option<String>,
}

impl PlannedCommand {
  fn display(&self) -> String {
    self.argv.join(" ")
  }
}

pub fn grant_access(target: Target) -> io::Result<()> {
  let info = inspect(target).map_err(io::Error::from)?;

  if info.is_root {
    println!("Already root, nothing to grant.");
    return Ok(());
  }
  if info.readable && info.writable {
    println!(
      "Access already granted to {}. Nothing to do.",
      target.label()
    );
    return Ok(());
  }

  let user = current_user().map_err(io::Error::from)?;
  let group = info
    .group
    .clone()
    .unwrap_or_else(|| target.default_group().to_string());

  let mut steps = Vec::new();

  if !info.group_exists {
    steps.push(PlannedCommand {
      explain: format!("Create the `{group}` group"),
      argv: vec!["sudo".into(), "groupadd".into(), group.clone()],
      stdin: None,
    });
  }

  // Device isn't gated by any real group yet; add a udev rule so it is.
  if info.group.is_none() {
    let rule_path = target.udev_rule_filename();
    steps.push(PlannedCommand {
      explain: format!("Write a udev rule so `{group}` gets access to the device ({rule_path})"),
      argv: vec!["sudo".into(), "tee".into(), rule_path.into()],
      stdin: Some(target.udev_rule_body(&group)),
    });
    steps.push(PlannedCommand {
      explain: "Reload udev rules".into(),
      argv: vec![
        "sudo".into(),
        "udevadm".into(),
        "control".into(),
        "--reload-rules".into(),
      ],
      stdin: None,
    });
    steps.push(PlannedCommand {
      explain: "Apply the new rule to devices already present".into(),
      argv: vec!["sudo".into(), "udevadm".into(), "trigger".into()],
      stdin: None,
    });
  }

  steps.push(PlannedCommand {
    explain: format!("Add {user} to the `{group}` group"),
    argv: vec![
      "sudo".into(),
      "usermod".into(),
      "-aG".into(),
      group.clone(),
      user.clone(),
    ],
    stdin: None,
  });

  println!(
    "To grant access to {}, this needs to run:\n",
    target.label()
  );
  for step in &steps {
    println!("  # {}", step.explain);
    println!("  $ {}", step.display());
    if let Some(stdin) = &step.stdin {
      for line in stdin.lines() {
        println!("    > {line}");
      }
    }
    println!();
  }
  println!("More info: {}", target.help_url());

  if !confirm("Run these now? [Y/n] ")? {
    println!("Exiting without changes");
    return Ok(());
  }

  match ensure_sudo() {
    Ok(true) => {}
    Ok(false) => {
      println!("sudo authentication failed - aborting.");
      return Ok(());
    }
    Err(e) => return Err(e.into()),
  }

  for step in &steps {
    if let Err(e) = run_planned(step) {
      eprintln!("\nFailed while trying to: {}", step.explain);
      eprintln!("  command: {}", step.display());
      eprintln!("  error: {e}");
      eprintln!("\nTo do this by hand see {}", target.help_url());
      return Err(e.into());
    }
  }

  println!(
    "\nDone. `{user}` is now in `{group}` - log out and back in (or run `newgrp {group}`) for it to take effect in the current shell."
  );
  if info.device.is_none() {
    match target {
      Target::Uinput => println!(
        "Note: /dev/uinput wasn't present. Make sure the `uinput` kernel module is loaded (`sudo modprobe uinput`), then re-run `rin perm --status`."
      ),
      Target::Input => println!(
        "Note: no /dev/input/event* device was present. The rule will still apply once devices update or a reboot is done."
      ),
    }
  }

  Ok(())
}

fn run_planned(step: &PlannedCommand) -> Result<(), CheckError> {
  let program = &step.argv[0];
  let args = &step.argv[1..];

  let mut cmd = Command::new(program);
  cmd.args(args);
  if step.stdin.is_some() {
    cmd.stdin(Stdio::piped());
  }

  let mut child = cmd.spawn().map_err(|e| match e.kind() {
    io::ErrorKind::NotFound => CheckError::BinaryNotFound(program.clone()),
    _ => CheckError::IoError(e),
  })?;

  if let Some(input) = &step.stdin {
    if let Some(mut stdin) = child.stdin.take() {
      stdin
        .write_all(input.as_bytes())
        .map_err(CheckError::IoError)?;
    }
  }

  let status = child.wait().map_err(CheckError::IoError)?;
  if !status.success() {
    return Err(CheckError::NonZeroExit {
      program: program.clone(),
      code: status.code().unwrap_or(-1),
    });
  }
  Ok(())
}

fn confirm(prompt: &str) -> io::Result<bool> {
  print!("{prompt}");
  io::stdout().flush()?;
  let mut line = String::new();
  io::stdin().read_line(&mut line)?;
  let line = line.trim().to_lowercase();
  Ok(line.is_empty() || line == "y" || line == "yes")
}

pub fn has_uinput_access() -> Result<bool, CheckError> {
  has_access(Target::Uinput)
}

pub fn has_input_access() -> Result<bool, CheckError> {
  has_access(Target::Input)
}

fn has_access(target: Target) -> Result<bool, CheckError> {
  if is_root()? {
    return Ok(true);
  }
  let info = inspect(target)?;
  Ok(info.readable && info.writable)
}
