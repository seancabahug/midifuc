use std::thread::current;

use midly::{num::u7, MidiMessage, TrackEvent, TrackEventKind};

#[derive(Debug)]
pub enum Operation {
    MovePointerRight,
    MovePointerLeft,
    ToggleIncDec,
    BeginLoop,
    EndLoop,
    Output,
    Input,
}

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub repetitions: u32,
    pub change_in_value: u32,
}

type Program = Vec<Instruction>;

pub fn parse(midi_track: &Vec<TrackEvent<'_>>) -> Program {
    let mut program: Vec<Instruction> = vec![];

    let mut enrolled_pitch_classes: Vec<i16> = vec![];
    let mut currently_pressed_keys: Vec<u7> = vec![];
    let mut pressed_keys_at_detection: Vec<u7> = vec![];
    let mut current_change_in_value: u16 = 0;

    for midi_event in midi_track {
        if midi_event.delta > 0 {
            // Nothing detected yet. Determine if keys pressed now has an octave
            if pressed_keys_at_detection.len() == 0 {
                if currently_pressed_keys.len() >= 2 {
                    for i in 0..currently_pressed_keys.len() - 1 {
                        let mut detected = false;

                        for j in i + 1..currently_pressed_keys.len() {
                            let first_key: i16 = currently_pressed_keys[i].as_int().into();
                            let second_key: i16 = currently_pressed_keys[j].as_int().into();

                            // Keys of the same pitch class are 12 apart
                            if (first_key - second_key) % 12 == 0 {
                                // instruction is detected
                                detected = true;

                                pressed_keys_at_detection = currently_pressed_keys.clone();

                                // Still in enrollment phase?
                                if enrolled_pitch_classes.len() < 8 {
                                    // A0 = MIDI note number 21
                                    let distance_from_a = (first_key - 21) % 12;
                                    let pitch_class_to_enroll =
                                        if enrolled_pitch_classes.contains(&distance_from_a) {
                                            -1
                                        } else {
                                            distance_from_a.into()
                                        };

                                    enrolled_pitch_classes.push(pitch_class_to_enroll);
                                    println!(
                                        "Enrolled {} as instruction {}",
                                        pitch_class_to_enroll,
                                        enrolled_pitch_classes.len()
                                    );

                                    if enrolled_pitch_classes.len() == 8 {
                                        println!("Finished enrollment phase, now parsing program");
                                    }
                                    break;
                                }

                                break;
                            }
                        }

                        if detected {
                            break;
                        }
                    }
                }

            // Instruction was already detected
            } else {
                // Determine if all keys in pressed_keys_at_detection have been released yet

                let mut key_still_pressed = false;

                for key in &pressed_keys_at_detection {
                    if currently_pressed_keys.contains(&key) {
                        key_still_pressed = true;
                        break;
                    }
                }

                if !key_still_pressed {
                    pressed_keys_at_detection = vec![];
                }
            }
        }

        if let TrackEventKind::Midi {
            channel: _,
            message,
        } = midi_event.kind
        {
            match message {
                MidiMessage::NoteOn { key, vel: _ } => {
                    currently_pressed_keys.push(key);
                }
                MidiMessage::NoteOff { key, vel: _ } => {
                    let index_to_remove = currently_pressed_keys
                        .iter()
                        .position(|k| *k == key)
                        .unwrap();
                    currently_pressed_keys.remove(index_to_remove);
                }
                _ => {}
            }
        }
    }

    program
}
