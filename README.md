# Signal to Noise

A Linux focus tool that helps reduce distractions by filtering your active window usage.

Signal to Noise monitors the current window, calculates a distraction score based on customizable keywords, and can automatically enable grayscale mode when distractions are detected.

The idea is simple:

**Keep the signal. Remove the noise.**

## Features

* Custom distraction keywords
* Custom productive keywords
* Adjustable scoring system
* Automatic grayscale mode
* GUI for managing keywords
* Persistent user settings
* Built with Rust

<p align="center">
  <img src="assets/screenshot.png" width="700">
</p>

## How it works

Signal to Noise checks the active window title and compares it against your keyword list.

Example:

```
YouTube - "CCNA VLAN Tutorial"

Score: -40
Status: Productive
```

```
TikTok - For You Page

Score: +50
Status: Distracting
```

When the distraction score reaches a certain threshold, grayscale mode is enabled.

## Configuration

Keywords can be customized through the settings file or the GUI.

Example:

'''
[distracting]
tiktok = 50
instagram = 40
shorts = 60

[productive]
coding = -30
rust = -30
ccna = -40
```

Positive values increase distraction.
Negative values increase focus.

## Installation

Clone the repository:

```bash
git clone https://github.com/Dynjee/Automatic-Anti-Distraction.git
cd FocusShade
```

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run --bin focusshade
```

GUI:

```bash
cargo run --bin focusshade-gui
```

## Requirements

* Linux
* Rust
* Hyprland (currently)
* Wayland

## Roadmap

Things I want to improve:

* Better word matching
* Application-based rules
* Browser extension support
* Focus statistics
* AI-based content classification
* Support for more desktop environments

## Why?

Most productivity tools block everything. I wanted something different: a system that lets you decide what is useful and what is distracting.

## License

MIT License

```

Built with Rust.
```
