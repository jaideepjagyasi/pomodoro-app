# Pomodoro TUI 🍅

A minimalist, terminal-based Pomodoro timer built with Rust. Focus on your work with structured intervals and breaks.

## Features

- ⏱️ **Pomodoro Timer**: customizable focus durations.
- ☕ **Break Management**: Automatic transitions between focus sessions, short breaks, and long breaks (every 4th session).
- 🔊 **Audio Notifications**: Sound alerts when timers start and end.
- 🖥️ **TUI Interface**: A clean, intuitive terminal user interface powered by `ratatui`.
- ⚙️ **On-the-fly Settings**: Adjust durations directly from the UI.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/pomodoro-app.git
   cd pomodoro-app
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

| Key | Action |
|-----|--------|
| `Space` / `Enter` | Start / Pause / Resume timer |
| `s` | Stop and reset timer |
| `k` | Skip to the next phase |
| `h` / `Left` | Previous tab/setting |
| `l` / `Right` | Next tab/setting |
| `+` / `=` | Increase current setting |
| `-` / `_` | Decrease current setting |
| `q` / `Esc` | Quit application |

## Project Structure

- `src/main.rs`: Application entry point and main loop.
- `src/timer.rs`: Timer logic and state management.
- `src/ui.rs`: TUI rendering and layouts.
- `src/audio.rs`: Sound playback implementation.

## License

This project is open source and available under the [MIT License](LICENSE).
