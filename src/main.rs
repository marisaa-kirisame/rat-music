extern crate rodio;
extern crate rfd;
extern crate id3;

use id3::{Tag, TagLike};
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader, Sink, Split, stderr};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::*;
use std::{io, thread};
use std::cmp::Ordering;
use std::ops::{Add, Deref, Index, Sub};
use std::path::Path;
use std::time::Duration;
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;
use rodio::*;
use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::widgets::{Block, Borders, Paragraph};

struct Song {
    title: String,
    author: String
}

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
struct ProgramState {
    exit: bool,
    mute: bool,
    volume: f32,
    current_playing: Option<Song>
}

impl Song {
    fn new(title: String, author: String) -> Song {
        Song {
            title,
            author
        }
    }

    fn new_from_file(path: Path) -> Option<Song> {
        let tag = Tag::read_from_path(path);
        return Some(Song{
            
        })
    }
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState {
            exit: false,
            mute: false,
            volume: 0.5,
            current_playing: None
        }
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}


fn clamp(value: f32, lower_bounds: f32, upper_bounds: f32) -> f32 {
    if value < lower_bounds {
        return lower_bounds;
    }
    else if value > upper_bounds {
        return upper_bounds;
    }

    return value;
}

fn main() -> Result<()> {
    // Create volume variable
    let mut volume: f32 = 0.5;
    let mut exit = false;

    // Initialize rodio stream and sink for audio playback
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    //let file1 = std::fs::File::open("assets/drill.mp3").unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    //sink.append(rodio::Decoder::new(BufReader::new(file1)).unwrap());

    startup()?;
    let status = run(sink);
    shutdown()?;
    status?;
    Ok(())
}

fn run(mut sink: rodio::Sink) -> Result<()> {
    // ratatui terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // app struct
    let mut state = ProgramState::new();

    loop {
        terminal.draw( |f| {
            ui(&mut state, f);
        })?;

        update(&mut sink, &mut state)?;

        if state.exit {
            break;
        }
    }

    return Ok(());
}

fn update(sink: &mut rodio::Sink, state: &mut ProgramState) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => sink.set_volume(sink.volume()+0.1), // raise volume
                    KeyCode::Down => sink.set_volume(sink.volume()-0.1), // lower vol
                    KeyCode::Char('q') => state.quit(),
                    KeyCode::Char('p') => {
                        let file_path_buffer = FileDialog::new()
                            .add_filter("music", &["mp3", "flac", "wav", "ogg"])
                            .set_directory("/")
                            .pick_file()
                            .unwrap();
                        let file = std::fs::File::open(file_path_buffer).unwrap();
                        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                    },
                    _ => {},
                }
            }
        }
    }
    Ok(())
}

fn ui(state: &mut ProgramState, frame: &mut Frame<'_>) {
    frame.render_widget(
        Paragraph::new("Now playing: nothing")
                .block(Block::default().title("Welcome to rat-music!").borders(Borders::all())),
                frame.size()
    );
}

fn startup() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

fn shutdown() -> Result<()> {
    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}