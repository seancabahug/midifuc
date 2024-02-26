use midly::TrackEvent;

pub enum Operation {
    MovePointerRight,
    MovePointerLeft,
    ToggleIncDec,
    BeginLoop,
    EndLoop,
    Output,
    Input,
}

pub struct Instruction {
    operation: Operation,
    repetitions: u32,
    change_in_value: u32,
}

pub type Program = Vec<Instruction>;

pub fn parse(midi_track: &Vec<TrackEvent<'_>>) -> Program {
    let program: Vec<Instruction> = vec![];



    program
}