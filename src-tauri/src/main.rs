// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::io::{stdin, stdout, Write};
use midir::{Ignore, MidiInput};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_device_config(handle: tauri::AppHandle) -> String {
    let config_path = handle.path_resolver()
        .resolve_resource("sampleData/deviceConfig.json")
        .expect("failed to read JSON");
    return std::fs::read_to_string(&config_path).expect("failed to read JSON");
}

fn parse_sysex_message(message: &[u8]) -> String {
    // We expect the message to be
    // F0 7D 6D 64 6C ... cs F7
    if message.len() > 0 
        && message[0] == 0xF0 
        && message[1] == 0x7D
        && message[2] == 0x6D
        && message[3] == 0x64
        && message[4] == 0x6C {
            let cs = calculate_checksum(message);
            if cs == message[message.len() - 1] {
                match message[5] {
                    0x00 => String::from("hello"),
                    0x01 => String::from("hello"),
                    _ => String::from("hello")
                }
            } else {
                String::from("hell")
            }
        } else {
            String::from("hell")
        }
}

fn calculate_checksum(message: &[u8]) -> u8 {
    let mut checksum = message[0];
    for i in 1..message.len()-2 {
        checksum ^= message[i]
    }
    checksum &= 0x7F;
    checksum
}

fn main() {
    run().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_device_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
                );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailabe input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
