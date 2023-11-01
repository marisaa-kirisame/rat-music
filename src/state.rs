use crate::song::Song;

pub struct ProgramState {
    pub exit: bool,
    pub mute: bool,
    pub volume: f32,
    pub speed: f32,
    pub current_playing: Option<Song>,
    pub queue: Vec<Song>
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            exit: false,
            mute: false,
            volume: 0.5,
            speed: 1.0,
            current_playing: None,
            queue: vec![]
        }
    }

    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn add_to_queue(&mut self, song: Song) {
        self.queue.push(song);
    }
}
