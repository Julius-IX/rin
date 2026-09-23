use std::{io, process::ExitCode, thread, time::Duration};

use clap::{CommandFactory, Parser};
use strum::IntoEnumIterator;

use crate::{
  devices::InputDevice,
  input_event_codes::{EventTypes, Key},
};

pub mod perm;

#[derive(Parser)]
pub struct Cli {
  /// Events to send allows listing multiple events (comma separated).
  /// - `K:0` sends `K` with value `0` (release only)
  /// - `K:1` sends `K` with value `1` (press only, held down)
  /// - `K:2` sends `K` with value `2` (autorepeat)
  /// - `K` sends `K` press and release
  /// - linux kernel input event code names
  #[arg(verbatim_doc_comment)]
  pub events: Vec<String>,

  /// List all event names
  #[clap(short, long)]
  pub list: bool,

  #[clap(subcommand)]
  pub perm: Option<perm::Perm>,
}

/// One parsed `events` entry: a key, plus the explicit value to send it
/// with (`None` means a full press-and-release, i.e. `1` then `0`).
#[derive(Debug, Clone, Copy)]
struct EventSpec {
  key: Key,
  value: Option<i32>,
}

impl EventSpec {
  fn parse(raw: &str) -> Result<Self, String> {
    let (name, value) = match raw.split_once(':') {
      Some((name, value)) => {
        let parsed = value.trim().parse::<i32>().map_err(|_| {
          format!("'{raw}': `:{value}` isn't a valid state (expected a number like 0, 1, or 2)")
        })?;
        (name, Some(parsed))
      }
      None => (raw, None),
    };

    let key =
      Key::from_name(name.trim()).ok_or_else(|| format!("'{name}': not a recognized key name"))?;

    Ok(EventSpec { key, value })
  }
}

pub fn run() -> ExitCode {
  let cli = Cli::parse();

  if cli.list {
    for event in Key::iter() {
      println!("{}", event.name());
    }
    return ExitCode::SUCCESS;
  }

  if let Some(cmd) = &cli.perm {
    return match cmd.run() {
      Ok(()) => ExitCode::SUCCESS,
      Err(e) => {
        eprintln!("error: {e}");
        ExitCode::FAILURE
      }
    };
  }

  if cli.events.is_empty() {
    Cli::command().print_help().unwrap();
    return ExitCode::FAILURE;
  }

  match cli.send() {
    Ok(()) => ExitCode::SUCCESS,
    Err(e) => {
      eprintln!("error: {e}");
      ExitCode::FAILURE
    }
  }
}

impl Cli {
  /// Splits every `events` entry on `,` (so both `rin K,L` and `rin K L`
  /// work) and parses each piece into an [`EventSpec`].
  fn parse_events(&self) -> Result<Vec<EventSpec>, String> {
    self
      .events
      .iter()
      .flat_map(|arg| arg.split(','))
      .map(str::trim)
      .filter(|s| !s.is_empty())
      .map(EventSpec::parse)
      .collect()
  }

  /// Opens a temporary virtual keyboard advertising exactly the keys
  /// `events` needs, sends them in order with the requested state, then
  /// tears the device back down.
  pub fn send(&self) -> io::Result<()> {
    let specs = self
      .parse_events()
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if specs.is_empty() {
      return Ok(());
    }

    let mut builder = InputDevice::builder("rin");
    for spec in &specs {
      builder = builder.key(spec.key);
    }
    let mut device = builder.build()?;

    // The kernel (and anything listening for new devices, e.g. libinput)
    // needs a moment to notice the device; sending events immediately
    // after creating it can be dropped on the floor.
    thread::sleep(Duration::from_millis(200));

    for spec in specs {
      match spec.value {
        Some(value) => device.send_event(EventTypes::Key, spec.key.code(), value)?,
        None => device.press_key(spec.key)?,
      }
    }

    // `device` is dropped here, issuing UI_DEV_DESTROY.
    Ok(())
  }
}
