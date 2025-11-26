# AI Prompt Journal: Learning Rust + egui via GenAI

Full detailed log of all 12 prompts used over 4 days while building the **Rust + egui Weather & Jokes Desktop App**.

---

## 📅 Day-by-Day Prompt Log

**Project:** Rust + egui Weather & Dad Jokes Desktop App

### Prompt 1 

**Goal:** Get a working egui window with heading and input field
**I was stuck on:** How to even start a native GUI in Rust (zero examples worked on WSL)

**Exact Prompt I used:**
I'm completely new to Rust GUI. I need a minimal working example using eframe 0.28 that:

Opens a native window
Has a centered heading "Weather + Dad Jokes"
Has a single-line text input
Works on WSL2 without crashing
Please give me the full main.rs and Cargo.toml. Do not use webview or iced.

**What the AI gave me:** Perfect working template + explained `NativeOptions` and `CentralPanel`
**Result:** First window appeared in 3 minutes
**Reflection:** Saved me ~4 hours of reading confusing docs

---

### Prompt 2 

**Goal:** Make the Enter key trigger the weather fetch
**I was confused about:** How to detect keypress inside `update()`

**Exact Prompt:**
In egui, how do I detect that the user pressed Enter while typing in a text_edit_singleline?
I tried ctx.input().key_pressed but it's not working.
Show me the exact code inside the update() function that checks for Enter AND allows button click.
Also prevent action if the input is empty.

**AI Fixed:** `ctx.input(|i| i.key_pressed(Key::Enter)) && !self.city.trim().is_empty()`
**Result:** UX became buttery smooth
**Learning:** egui input reading pattern

---

### Prompt 3 

**Goal:** Fetch weather without freezing the UI
**Error I was getting:** `there is no reactor running, must be called from the context of a Tokio runtime`

**Exact Prompt (Root Cause Analysis style):**
I need help diagnosing the root cause of this panic in my egui app:
thread '...' panicked at ... hyper-util-0.1.18/src/client/legacy/connect/dns.rs:119:24:
there is no reactor running, must be called from the context of a Tokio 1.x runtime
Code snippet:

```
std::thread::spawn(move || {
let result = pollster::block_on(fetch_weather(&city));  // ← panics here
});
```

I have tokio = { features = ["full"] } in Cargo.toml
This error occurs every time I click "Get Weather"
I already tried removing pollster and using tokio::runtime but same issue.
Please:

Tell me the real root cause
Give me the correct way to do background HTTP in eframe 0.28
Show the exact working thread code

**AI Answer:** Explained that reqwest + tokio tries to use global runtime → conflict with egui → must use blocking or proper thread
**Fix I applied:** Switched to `reqwest::blocking` (final working version)
**Reflection:** This single prompt saved me 2 full days

---

### Prompt 4

**Error:** `Broken pipe (os error 32)` when closing window

**Exact Prompt (Root Cause style):**
My egui app crashes on close in WSL2 with:
Io error: Broken pipe (os error 32)
It works fine for 10 seconds then crashes when I close the window.
I am using eframe 0.28 on Ubuntu 22.04 in WSL2 with X11 forwarding.
Please give me the exact environment variable fix and where to put it.

**AI Answer:** `std::env::set_var("WINIT_UNIX_BACKEND", "x11");` at start of main
**Result:** Zero crashes after this
**Reflection:** GenAI knew WSL-specific quirks better than 10 Stack Overflow threads

---

### Prompt 5

**Problem:** Dad jokes were changing every frame (flickering)

**Exact Prompt:**
In my egui app, this code causes the joke to change every frame:

```
let mut joke_index = 0;
if ui.button("New Joke").clicked() { joke_index += 1; }
ui.label(jokes[joke_index % jokes.len()]);
```

How do I make a global counter that only changes when the button is clicked?
Please show me the AtomicUsize pattern that works in egui.

**AI gave me:** The exact `static JOKE_INDEX: AtomicUsize` code you now have
**Result:** Perfect stable jokes

---

### Prompt 6

**Goal:** Pass weather result from background thread to UI

**Exact Prompt:**
In egui, I have a background thread that fetches weather as String.
How do I safely pass that String back to the UI on the next frame?
I tried global static mutex but it felt wrong.
Please show me the correct egui way using Context or memory.

**AI Answer:** `ctx.memory_mut(|mem| mem.data.insert_temp(...))` + `get_temp`
**Result:** Clean, idiomatic state passing

---



