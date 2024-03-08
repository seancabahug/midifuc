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
    SpecialInstruction,
}

pub fn index_to_operation(i: usize) -> Option<Operation> {
    match i {
        0 => Some(Operation::MovePointerRight),
        1 => Some(Operation::MovePointerLeft),
        2 => Some(Operation::ToggleIncDec),
        3 => Some(Operation::BeginLoop),
        4 => Some(Operation::EndLoop),
        5 => Some(Operation::Output),
        6 => Some(Operation::Input),
        7 => Some(Operation::SpecialInstruction),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub repetitions: u32,
    pub change_in_value: u32,
}

struct MidiEventCluster {
    note_offs: Vec<MidiMessage>,
    note_ons: Vec<MidiMessage>,
}

type Program = Vec<Instruction>;

fn clusterize_midi_events<'a>(midi_track: &Vec<TrackEvent<'a>>) -> Vec<MidiEventCluster> {
    let mut midi_event_clusters: Vec<MidiEventCluster> = vec![];
    let mut current_note_offs: Vec<MidiMessage> = vec![];
    let mut current_note_ons: Vec<MidiMessage> = vec![];

    for midi_event in midi_track {
        if let TrackEventKind::Midi {
            channel: _,
            message,
        } = midi_event.kind
        {
            match message {
                MidiMessage::NoteOn { key: _, vel: _ }
                | MidiMessage::NoteOff { key: _, vel: _ } => {
                    // maybe a worry: if midi event types other than note_on/note_off occur, midi_event.delta will not accurately give a delta between a note_on/note_off event
                    if midi_event.delta > 0
                        && (current_note_offs.len() > 0 || current_note_ons.len() > 0)
                    {
                        let cluster = MidiEventCluster {
                            note_offs: current_note_offs.clone(),
                            note_ons: current_note_ons.clone(),
                        };
                        midi_event_clusters.push(cluster);

                        current_note_offs = vec![];
                        current_note_ons = vec![];
                    }

                    match message {
                        MidiMessage::NoteOn { key: _, vel: _ } => current_note_ons.push(message),
                        MidiMessage::NoteOff { key: _, vel: _ } => current_note_offs.push(message),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    // the last event cluster doesn't get pushed yet, doing that now
    let cluster = MidiEventCluster {
        note_offs: current_note_offs.clone(),
        note_ons: current_note_ons.clone(),
    };
    midi_event_clusters.push(cluster);

    midi_event_clusters
}

pub fn parse(midi_track: &Vec<TrackEvent<'_>>) -> Program {
    let mut program: Vec<Instruction> = vec![];

    let mut enrolled_pitch_classes: Vec<i16> = vec![];

    let mut pressed_keys_at_detection: Vec<u7> = vec![];
    let mut currently_pressed_keys: Vec<u7> = vec![];
    let mut currently_detected_operation: Option<Operation> = None;
    let mut current_change_in_value: u32 = 0;
    let mut current_repetitions: u32 = 0;

    for cluster in clusterize_midi_events(midi_track) {
        for event in cluster.note_offs {
            let MidiMessage::NoteOff { key, vel: _ } = event else { continue };

            let index_to_remove = currently_pressed_keys
                .iter()
                .position(|k| *k == key)
                .unwrap();
            currently_pressed_keys.remove(index_to_remove);

            let mut key_still_pressed = false;

            for key in &pressed_keys_at_detection {
                if currently_pressed_keys.contains(&key) {
                    key_still_pressed = true;
                    break;
                }
            }

            if !key_still_pressed {
                pressed_keys_at_detection = vec![];

                if enrolled_pitch_classes.len() >= 8 && currently_detected_operation.is_some() {
                    let instruction = Instruction {
                        operation: currently_detected_operation.unwrap(),
                        repetitions: current_repetitions,
                        change_in_value: current_change_in_value,
                    };

                    dbg!(&instruction);

                    program.push(instruction);

                    currently_detected_operation = None;
                }

                current_repetitions = 0;
                current_change_in_value = 0;
            }
        }

        for event in cluster.note_ons {
            let MidiMessage::NoteOn { key, vel: _ } = event else { continue };

            if !currently_pressed_keys.contains(&key) {
                currently_pressed_keys.push(key);
            }

            if enrolled_pitch_classes.len() >= 8 && pressed_keys_at_detection.len() > 0 {
                if pressed_keys_at_detection.contains(&key) {
                    current_repetitions += 1;
                } else {
                    current_change_in_value += 1;
                }
            }
        }

        if currently_pressed_keys.len() >= 2 && pressed_keys_at_detection.len() == 0 {
            for i in 0..currently_pressed_keys.len() - 1 {
                let mut detected = false;

                for j in (i + 1)..currently_pressed_keys.len() {
                    let first_key: i16 = currently_pressed_keys[i].as_int().into();
                    let second_key: i16 = currently_pressed_keys[j].as_int().into();

                    // Keys of the same pitch class are 12 apart
                    if (first_key - second_key) % 12 == 0 {
                        // instruction is detected
                        detected = true;

                        pressed_keys_at_detection = currently_pressed_keys.clone();

                        // A0 = MIDI note number 21
                        // used in both enrollment and program phases
                        let distance_from_a = (first_key - 21) % 12;

                        // Still in enrollment phase?
                        if enrolled_pitch_classes.len() < 8 {
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
                        } else {
                            // Currently in program phase
                            let index_of_operation = enrolled_pitch_classes
                                .iter()
                                .position(|&p| p == distance_from_a)
                                .unwrap();
                            currently_detected_operation = index_to_operation(index_of_operation);
                        }

                        break;
                    }
                }

                if detected {
                    break;
                }
            }
        }
    }

    program
}