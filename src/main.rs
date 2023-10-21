extern crate rodio;
extern crate tokio;

use std::fs::File;
use std::io::{BufReader, Split};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::*;
use std::{io, thread};
use std::cmp::Ordering;
use std::ops::{Add, Deref, Index, Sub};
use std::path::Path;
use std::time::Duration;
use ratatui::buffer::Buffer;
use rodio::*;

// TODO: Figure out tokio or asynchronicity, I'm going to sleep

fn clamp(value: f32, lower_bounds: f32, upper_bounds: f32) -> f32 {
    if value < lower_bounds {
        return lower_bounds;
    }
    else if value > upper_bounds {
        return upper_bounds;
    }

    return value;
}

fn main() {

    // Create volume variable
    let mut volume: f32 = 0.5;
    let mut exit = false;


    // Initialize rodio stream and sink for audio playback
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    //let file1 = std::fs::File::open("assets/drill.mp3").unwrap();
    //let file2 = std::fs::File::open("assets/everyone.mp3").unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    //sink.append(rodio::Decoder::new(BufReader::new(file1)).unwrap());

    loop {
        // Read line
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error while reading string.");

        // Clean input up
        input = input.replace("\r", "");
        input = input.replace("\n", "");

        // Separate input into commands
        let mut command: Vec<&str> = input.split(' ').collect();

        println!("Command is: {:?}\n", command);

        // Match input
        match command[0] {
            // Quit on Q
            "q" | "Q" | "quit" => {break},
            // Mute on M
            "m" | "M" | "mute" => {
                if sink.volume() > 0.0 {
                    sink.set_volume(0.0);
                    println!("(i) Muted.\n")
                }
                else {
                    sink.set_volume(1.0);
                    println!("(i) Unmuted.\n")
                }

            },
            "v" | "V" | "volume" => {
                if command.len() >= 2 {
                    sink.set_volume(match command[1].parse() {
                        Ok(x) => clamp(x, 0.0, 1.0),
                        Err(_) => sink.volume(),
                    });
                   println!("(i) Volume set to {}\n", sink.volume())
                };
            },
            "p" | "P" | "play" => {
                if command.len() >= 2{
                    let file = File::open(command[1]);
                    match file {
                        Ok(f) => {
                            println!("(i) {:?} queued. \n", f);
                            sink.append(Decoder::new(BufReader::new(f)).unwrap());

                        }
                        Err(_) => {
                            println!("(!) Couldn't open file.\n")
                        }
                    }

                }
            },
            "h" | "H" | "help" | "HELP" | "what" => {
                println!(
"--- --- ---\n
Welcome to rat-music! The following commands can be used to operate the program:\n
- q/Q/quit -- Quits the program\n
- m/M/mute -- Mutes and unmutes audio\n
- v/V/volume [vol] -- sets the volume from 0.0 to 1.0\n
- p/P/play [path] -- adds the file at [path] to the queue\n
--- --- ---\n")
            }
            _ => ()
        }
    }





}
