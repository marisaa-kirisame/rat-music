# rat music
Simple commandline UI music player I'm making to learn Rust. Uses [rodio](https://github.com/RustAudio/rodio) for audio playback and [ratatui](https://github.com/ratatui-org/ratatui) for TUI. Work in progress, please be patient.

# Basic operation of the program
In the current state, the music player features 3 main controls:
| Key | Function |
| --- | --- |
| q | Quits. |
| p | Opens the dialog for selecting a song. Might not focus the dialog window. |
| ↑ | Raises volume by 0.1 |
| ↓ | Lowers volume by 0.1 |
