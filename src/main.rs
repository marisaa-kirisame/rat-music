extern crate rodio;
extern crate rfd;
extern crate id3;

use std::fmt::format;
use id3::TagLike;
use rfd::FileDialog;
use std::io::{BufRead, BufReader, stderr};
use std::ops::{Add, Deref, Index, Sub};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rodio::*;
use anyhow::Result;
use crossterm::event::{self, KeyCode};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use song::Song;
use state::ProgramState;

mod song;
mod state;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;


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

    let sink = rodio::Sink::try_new(&handle).unwrap();

    startup()?;
    let status = run(sink);
    shutdown()?;
    status?;
    Ok(())
}

fn run(mut sink: rodio::Sink) -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

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

    // TODO: The song title in ProgramState doesn't change when the song ends!
    // Detect whenever the sink swaps the playing song and update the queue accordingly.
    // Possible solution is to extend the Sink struct with a function that lets the user view the queue.

    if event::poll(std::time::Duration::from_millis(250))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => sink.set_volume(sink.volume()+0.1),
                    KeyCode::Down => sink.set_volume(sink.volume()-0.1),
                    KeyCode::Char('q') => state.quit(),
                    KeyCode::Char('p') => {
                        let file_path_buffer = FileDialog::new()
                            .add_filter("music", &["mp3", "flac", "wav", "ogg"])
                            .set_directory("/")
                            .pick_file();
                        if file_path_buffer.is_some()
                        {
                            let file_path_buffer = file_path_buffer.unwrap();
                            let file = std::fs::File::open(file_path_buffer.clone()).unwrap();
                            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                            state.add_to_queue(Song::new_from_file(file_path_buffer.into_boxed_path()));
                        }
                    },
                    KeyCode::Char('s') => {
                        sink.skip_one();
                        if state.queue.len() > 0
                        { state.queue.remove(0); }
                    },
                    KeyCode::Char('h') => sink.set_speed(sink.speed() + 0.1),
                    KeyCode::Char('j') => sink.set_speed(sink.speed() - 0.1),
                    KeyCode::Char(' ') => {
                        if sink.is_paused() {
                            sink.play();
                        }
                        else {
                            sink.pause();
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    state.volume = sink.volume();
    state.speed = sink.speed();

    Ok(())
}

fn ui(state: &mut ProgramState, frame: &mut Frame<'_>) {
    let track_name: String = match &state.queue.get(0) {
        None => String::from("Nothing"),
        Some(song) => song.as_str()
    };

    let mut vol_position: Rect = Default::default();
    vol_position.x = frame.size().x + 1;
    vol_position.y = frame.size().y + 1;
    frame.render_widget(
        Paragraph::new(format!("Now playing: {}", track_name ))
                .block(Block::default().title("Welcome to rat-music!").borders(Borders::all())),
                frame.size()
    );

    frame.render_widget(Paragraph::new(format!("Vol: {}", state.volume.trunc())), Rect{x: 1, y:2, width: 8, height: 1});
    frame.render_widget(Paragraph::new(format!("Speed: {}", state.speed.trunc())), Rect{x: 1, y:3, width: 14, height: 1});
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
