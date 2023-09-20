use std::path::PathBuf;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}

fn dmw3_to_ascii(byte: u8) -> char {
    match byte {
        0x01 => ' ',
        0x04 => '0',
        0x05 => '1',
        0x06 => '2',
        0x07 => '3',
        0x08 => '4',
        0x09 => '5',
        0x0a => '6',
        0x0b => '7',
        0x0c => '8',
        0x0d => '9',
        0x0e => 'A',
        0x0f => 'B',
        0x10 => 'C',
        0x11 => 'D',
        0x12 => 'E',
        0x13 => 'F',
        0x14 => 'G',
        0x15 => 'H',
        0x16 => 'I',
        0x17 => 'J',
        0x18 => 'K',
        0x19 => 'L',
        0x1a => 'M',
        0x1b => 'N',
        0x1c => 'O',
        0x1d => 'P',
        0x1e => 'Q',
        0x1f => 'R',
        0x20 => 'S',
        0x21 => 'T',
        0x22 => 'U',
        0x23 => 'V',
        0x24 => 'W',
        0x25 => 'X',
        0x26 => 'Y',
        0x27 => 'Z',
        0x28 => 'a',
        0x29 => 'b',
        0x2a => 'c',
        0x2b => 'd',
        0x2c => 'e',
        0x2d => 'f',
        0x2e => 'g',
        0x2f => 'h',
        0x30 => 'i',
        0x31 => 'j',
        0x32 => 'k',
        0x33 => 'l',
        0x34 => 'm',
        0x35 => 'n',
        0x36 => 'o',
        0x37 => 'p',
        0x38 => 'q',
        0x39 => 'r',
        0x3a => 's',
        0x3b => 't',
        0x3c => 'u',
        0x3d => 'v',
        0x3e => 'w',
        0x3f => 'x',
        0x40 => 'y',
        0x41 => 'z',
        _ => ' ',
    }
}

fn translate(path: PathBuf) {
    let file = fs::read(&path).unwrap();

    let new_file = file.iter().map(|x| dmw3_to_ascii(*x)).collect::<String>();

    let _ = fs::write(
        format!("output/{}.txt", path.file_name().unwrap().to_str().unwrap()),
        new_file,
    );
}

fn main() {
    let args = Args::parse();

    let metadata = fs::metadata(&args.file).unwrap();

    if metadata.is_file() {
        translate(args.file);
    } else {
        if let Ok(entries) = fs::read_dir(args.file) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            translate(entry.path());
                        }
                    }
                }
            }
        }
    }
}
