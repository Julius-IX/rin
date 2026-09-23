// Linux kernel ports
mod input;
mod input_event_codes;
mod ioctl;
mod uinput;

// Other
mod cli;
mod devices;

use std::process::ExitCode;

fn main() -> ExitCode {
  cli::run()
}
