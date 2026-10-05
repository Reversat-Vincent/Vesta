//! Vesta binary entry point; all logic lives in the `vesta` library.

// Release builds on Windows use the GUI subsystem, so no console window opens; debug builds keep
// the console to show diagnostic output.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
    vesta::run()
}
