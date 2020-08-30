use std::env;
use std::fs;
use std::fs::File;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
// use std::path::Path;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];

    let current_dir = env::current_dir();
    fs::create_dir_all("output");

    for entry in fs::read_dir(".")? {
        let dir = entry?;
        if dir.file_type()?.is_dir() {
            continue;
        }
        if let Some(ext) = dir.path().extension() {
            if ext != "log" {
                continue;
            }
        }
        parse_file(&dir.path().to_str().unwrap());
    }
    Ok(())
}

fn parse_file(filename: &str) {
    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    // println!("With text:\n{}", contents);

    let mut chat_lines = Vec::new();
    let mut unknown_identifiers = HashSet::new();
    let lines = contents.lines();
    for line in lines {
        let components: Vec<&str> = line.split("|").collect();
        let mut is_unknown = false;

        if components.len() < 4 {
            continue;
        }

        let channel_code = components[2];
        let channel = match channel_code {
            "000a" => Some("Say"),
            "000b" => Some("Shout"),
            "000c" => Some("Tell (Sent)"),
            "000d" => Some("Tell (Received)"),
            "000e" => Some("Party"),
            "000f" => Some("Alliance"), // guessing
            "001b" => None, // Novice Network
            "001c" => Some("Custom Emote"),
            "001d" => Some("Emote"),
            "001e" => Some("Yell"),
            "0018" => Some("Fc"),
            "003d" => None, // Retainer
            "0044" => None, // NPC
            "0010" => Some("LS1"),
            "0011" => Some("LS2"),
            // "0012" => Some("LS3"), // guessing
            // "0013" => Some("LS4"), // guessing
            "0014" => Some("LS5"),
            // "0015" => Some("LS6"), // guessing
            // "0016" => Some("LS7"), // guessing
            // "0017" => Some("LS8"), // guessing
            "0025" => Some("CWLS1"),
            "0065" => Some("CWLS2"),
            // "0066" => Some("CWLS3"), // guessing
            "0067" => Some("CWLS4"),
            // "0068" => Some("CWLS5"), // guessing
            "0069" => Some("CWLS6"),
            // "0070" => Some("CWLS7"), // guessing
            // "0071" => Some("CWLS8"), // guessing
            x if x.len() == 4 => {
                is_unknown = true;
                Some(x)
            },
            _ => None,
        };

        if channel.is_none() {
            continue;
        }

        let timestamp = components[1];
        let sender = components[3];
        let message = components[4];

        if sender.len() <= 1 {
            continue;
        }

        if is_unknown {
            unknown_identifiers.insert(channel_code);
        }
        writeln!(&mut chat_lines, "{}|{}|{}|{}", timestamp, channel.unwrap(), sender, message).unwrap();
    }

    let mut f2 = File::create("output/".to_owned() + &filename.to_owned()).expect("file already exists");
    f2.write(&unknown_identifiers.into_iter().collect::<Vec<&str>>().join("\n").into_bytes()).unwrap();
    f2.write(&chat_lines).unwrap();
}