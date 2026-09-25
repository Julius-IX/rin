# rin

Shittier `ydotool`, but this one doesn't crash my Hyprland setup.

`rin` is a small CLI tool that synthesizes keyboard input on Linux via `uinput`. It spins up a temporary virtual keyboard, sends specified events, and tears it back down.

## Install

```sh
git clone https://github.com/Julius-IX/rin.git
cd rin/
cargo install --path .
```

## Usage

```sh
# press and release a key
rin K

# type out a literal string (US QWERTY)
rin --type "hello world"

# multiple events, comma or space separated
rin K,L,ENTER
rin K L ENTER

# explicit key state: 0 = release, 1 = press (held), 2 = autorepeat
rin K:1
rin K:0

# delay between events, in seconds
rin --interval 0.05 K,L,ENTER

# list every recognized key name
rin --list
```

`--type` and positional `events` can be combined; `--type` is always sent first.

## Permissions

Sending events requires access to `/dev/uinput`. Check and fix that with:

```sh
# show current permission status
rin perm --status

# add yourself to the uinput group
rin perm --uinput

```

You may need to log out and back in (or reboot) for new group membership to take effect.

## As a library

`rin` also ships as a crate, exposing the ported Linux kernel constants and structs it's built on (`input_event_codes`, `input`, `uinput`, `ioctl`, and `devices`).

```toml
[dependencies]
rin = { git = "https://github.com/Julius-IX/rin.git" }
```

```rust
use rin::input_event_codes::Key;
use rin::devices::InputDevice;
```

See `src/lib.rs` for the full module list.

## Packaging

I'd like to publish this on the AUR, but AUR account registration is currently down because they're getting hammered by bot signups. That'll happen once registration is back open.

Not publishing to crates.io, on purpose. I have personal gripes with it. Use the git dependency above if you want it as a library.

## Todo / Future Features

- [ ] Better Mouse support: relative movement, clicks, and scroll (the `relative_axis` codes are already ported, just not wired up to the CLI yet)
- [ ] Absolute positioning (touchscreens, tablets, absolute-mode pointers)
- [ ] Gamepad/joystick emulation
- [ ] Non-US keyboard layouts for `--type` (currently hardcoded to QWERTY)
- [ ] AUR package (blocked on AUR registration reopening, see Packaging)
- [ ] Better error messages when a key name is typo'd or doesn't exist

## Why

`ydotool` works, but its daemon has a habit of taking my Hyprland setup down with it. `rin` does the same basic job, synthetic key events through `uinput`, just without the crashes.
