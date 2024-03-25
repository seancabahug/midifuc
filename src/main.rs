use midifuc::{interpreter::interpret, parser};
use midly::Smf;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Expected usage: {} <location-to-midi-file> <track-number>", args[0]);
        return;
    }

    let midi_bytes = fs::read(&args[1]).unwrap();
    let smf = Smf::parse(&midi_bytes).unwrap();

    let track_number: usize = args[2].parse().unwrap();
    let program = parser::parse(&smf.tracks[track_number]);
    interpret(program);
}
