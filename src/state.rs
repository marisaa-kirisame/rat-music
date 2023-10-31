use crate::song::Song;

pub struct ProgramState {
    pub exit: bool,
    pub mute: bool,
    pub volume: f32,
    pub current_playing: Option<Song>
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            exit: false,
            mute: false,
            volume: 0.5,
            current_playing: None
        }
    }

    pub fn quit(&mut self) {
        self.exit = true;
    }
}
