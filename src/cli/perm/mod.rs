use std::io;

use clap::{Args, CommandFactory, Parser, Subcommand};

pub mod checks;

#[derive(Subcommand)]
pub enum Perm {
  /// Inspect or fix the permissions needed to create/read input devices
  Perm(PermArgs),
}

impl Perm {
  pub fn run(&self) -> io::Result<()> {
    match self {
      Perm::Perm(args) => args.run(),
    }
  }
}

#[derive(Args)]
pub struct PermArgs {
  /// Show current permission status for uinput and input access
  #[arg(short, long)]
  status: bool,

  /// Add this user to the group needed to create virtual devices via uinput
  #[arg(short, long)]
  uinput: bool,

  /// Add this user to the group needed to read /dev/input/* devices
  #[arg(short, long)]
  input: bool,
}

impl PermArgs {
  pub fn run(&self) -> io::Result<()> {
    if !(self.status || self.uinput || self.input) {
      let mut cmd = super::Cli::command();
      cmd.find_subcommand_mut("perm").unwrap().print_help()?;
      return Ok(());
    }
    if self.status {
      checks::print_status();
    }
    if self.uinput {
      checks::grant_access(checks::Target::Uinput)?;
    }
    if self.input {
      checks::grant_access(checks::Target::Input)?;
    }
    Ok(())
  }
}
