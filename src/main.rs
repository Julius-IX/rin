// Linux kernel ports
mod input;
mod input_event_codes;
mod ioctl;
mod uinput;

// Other
mod cli;
mod devices;
mod permissions;

use clap::Parser;
use std::process::ExitCode;
use strum::IntoEnumIterator;

fn main() -> ExitCode {
  let cli = crate::cli::Cli::parse();
  if cli.list {
    for event in input_event_codes::Key::iter() {
      println!("{}", event.name());
    }
    return ExitCode::SUCCESS;
  };

  ExitCode::FAILURE
}

fn send_event(args: Vec<String>) -> ExitCode {
  todo!()
}
