# Rust egui Beginner's Toolkit: Building a Native Desktop Weather & Jokes App

## 1. Title & Objective

**Technology Chosen:** Rust + egui (immediate‑mode GUI library).
**Why This Tech?** As a Python developer learning systems programming, Rust offers memory safety + speed, and egui provides a lightweight GUI without JavaScript or heavy dependencies.
**Goal:** Build a functional native GUI that fetches weather data and displays dad jokes, demonstrating async Rust, UI state, and threading.

---

## 2. Quick Summary of the Technology

**What is Rust?** A systems programming language focusing on performance and safety without garbage collection.
**What is egui?** A pure‑Rust immediate‑mode GUI library that redraws every frame. It's simple, modern, and used in 2025 for editors, dashboards, and tools.

**Industry Usage:**

* Rust: AWS Firecracker, Dropbox sync engine.
* egui: Bevy Engine tools, i3bar-rs, embedded dashboards.

**Real Project Example:** *i3bar-rs* uses egui to render a dynamic desktop status bar.

---

## 3. System Requirements

**OS:** Linux (WSL2), macOS 12+, Windows 10+.
**Tools:** rustup (Rust 1.75+), Cargo, VS Code + rust-analyzer.
**Hardware:** 4GB RAM, internet for API calls.
**Dependencies:** Automatically fetched (eframe, egui, reqwest, serde).

---

## 4. Installation & Setup Instructions

Detailed instructions in `QUICKSTART.md`. Key commands:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone & Run
git clone https://github.com/luckyantony/weather-jokes-gui.git
cd weather-jokes-gui
cargo run
```

---

## 5. Minimal Working Example

**Core file:** `src/main.rs`
**How it works:**

* `update()` function draws the UI every frame.
* Weather fetching runs in a background thread.
* Jokes use an `AtomicUsize` counter to prevent flicker.

**Expected Output:**

* A window with temperature like **“22.5°C • Clear sky”**.
* Joke such as **“Why don’t skeletons fight each other? They don’t have the guts.”**

Run:

```bash
cargo run
```

---

## 6. AI Prompt Journal

Prompts used (12 prompts, 4 days) on ai.moringaschool.com:

| #  | Day | Prompt Summary             | Key Feature            | Gain         |
| -- | --- | -------------------------- | ---------------------- | ------------ |
| 1  | Mon | Simple egui window + input | Scaffold               | Saved 30min  |
| 2  | Mon | Text input + Enter         | Interactive UI         | —            |
| 3  | Tue | Async HTTP, non-blocking   | UI stays responsive    | Breakthrough |
| 4  | Tue | API error handling         | Red error text         | Bug fixes    |
| 5  | Wed | Spinner + bold text        | UI polish              | Better UX    |
| 6  | Wed | Atomic counter for jokes   | No flicker             | Smooth state |
| 7  | Wed | Fix WSL “Broken pipe”      | WINIT_UNIX_BACKEND=x11 | No crashes   |
| 8  | Thu | State across UI frames     | insert_temp            | Stable data  |
| 9  | Thu | WMO → description          | Human text             | Fun output   |
| 10 | Thu | Window size control        | Better layout          | Polish       |
| 11 | Fri | Vertical layout            | Final structure        | Consistent   |
| 12 | Fri | Moringa Markdown template  | Docs auto-written      | 70% speed-up |

---

## 7. Common Issues & Fixes

| Issue                       | Fix                            |
| --------------------------- | ------------------------------ |
| WSL window crashes on close | Set `WINIT_UNIX_BACKEND=x11`   |
| “City not found”            | Trim input + correct spelling  |
| Jokes changing every frame  | Use `AtomicUsize` + button     |
| UI freezing during fetch    | `std::thread::spawn` + repaint |

---

## 8. References

* *The Rust Book*
* egui documentation & demos
* Open-Meteo API
* ai.moringaschool.com prompts

---

**Moringa AI Capstone • Luckyantony Leshan • November 25, 2025**
