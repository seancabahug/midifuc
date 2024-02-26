use midifuc::parser::{parse, Operation};
use midly::Smf;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected usage: {} <location-to-midi-file>", args[0]);
        return;
    }

    let midi_bytes = fs::read(&args[1]).unwrap();
    let smf = Smf::parse(&midi_bytes).unwrap();

    let program = parse(&smf.tracks[1]);
    for instruction in program {
        dbg!(&instruction);
    }
}
