# Quickstart: Run the Rust egui Weather + Jokes GUI

This guide gets you from zero to running in under 5 minutes. Assumes Linux/WSL (adapt for macOS/Windows).

## Prerequisites

* **OS**: Ubuntu 22.04+ (WSL) or equivalent.
* **Rust**: 1.75+ (check with `rustc --version`).
* **Editor**: VS Code with rust-analyzer extension.
* **Internet**: Required for API fetches.

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustup update stable
```

## Step-by-Step Setup

### 1. Clone the Repository

```bash
git clone https://github.com/luckyantony/weather-jokes-gui.git
cd weather-jokes-gui
```

### 2. Fix WSL Display (WSL users only)

Add to your `~/.bashrc`:

```bash
export WINIT_UNIX_BACKEND=x11
```

Reload:

```bash
source ~/.bashrc
```

### 3. Build Dependencies

```bash
cargo check   # Downloads crates (eframe, reqwest, etc.)
```

### 4. Run the App

```bash
cargo run
```

### 5. Test Features

* Type **"London"** → press Enter → see weather like **"8.5°C • Cloudy"**.
* Click **"New Joke"** → joke changes.

### 6. Build Release Binary

```bash
cargo build --release
```

Run the binary:

* **Linux:** `./target/release/weather-jokes-gui`
* **Windows:** `target\release\weather-jokes-gui.exe`

## Troubleshooting

**Window not opening?**
Run this in WSL:

```bash
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0
```

**API error?**
Check your internet. "City not found" = misspelled city name.

---

Full technical details available in `PROJECT_SUMMARY.md`. Happy coding! 🦀
