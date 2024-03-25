use crate::parser::{Instruction, Operation};
use std::collections::HashMap;
use std::{io, io::Read};

fn find_loop_destinations(program: &Vec<Instruction>) -> HashMap<usize, usize> {
    let mut loop_destinations = HashMap::new();

    let mut loop_stack: Vec<usize> = vec![];
    for i in 0..(program.len()) {
        match program[i].operation {
            Operation::BeginLoop => {
                loop_stack.push(i);
            }
            Operation::EndLoop => {
                let corresponding_begin_loop_index = loop_stack.pop().unwrap();

                loop_destinations.insert(i, corresponding_begin_loop_index);
                loop_destinations.insert(corresponding_begin_loop_index, i);
            }
            _ => {}
        }
    }

    loop_destinations
}

pub fn interpret(program: Vec<Instruction>) {
    let mut memory_cells = [0; u16::MAX as usize];
    let mut memory_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;
    let mut incrementing = true;
    let mut increment_state_stack: Vec<bool> = vec![];

    println!("INTERPRETING");

    let loop_destinations = find_loop_destinations(&program);
    dbg!(&loop_destinations);

    while instruction_pointer < program.len() {
        let instruction = &program[instruction_pointer];

        for _ in 0..(instruction.repetitions + 1) {
            match instruction.operation {
                Operation::MovePointerRight => {
                    memory_pointer += 1;
                }
                Operation::MovePointerLeft => {
                    memory_pointer -= 1;
                }
                Operation::ToggleIncDec => {
                    incrementing = !incrementing;
                }
                Operation::BeginLoop => {
                    if memory_cells[memory_pointer] == 0 {
                        instruction_pointer = *loop_destinations.get(&instruction_pointer).unwrap();
                    } else {
                        increment_state_stack.push(incrementing);
                    }
                }
                Operation::EndLoop => {
                    if memory_cells[memory_pointer] != 0 {
                        instruction_pointer = *loop_destinations.get(&instruction_pointer).unwrap();
                        incrementing = increment_state_stack.last().unwrap().clone();
                    } else {
                        increment_state_stack.pop();
                    }
                }
                Operation::Output => {
                    print!("{}", char::from_u32(memory_cells[memory_pointer]).unwrap());
                }
                Operation::Input => {
                    let value = io::stdin().bytes().next();
                    if let Some(Ok(v)) = value {
                        memory_cells[memory_pointer] = v.into();
                    } else {
                        // something's wrong with input, either no more characters to read or something else. terminate here
                        return;
                    }
                }
                Operation::SpecialInstruction => {}
            }
        }

        if incrementing {
            memory_cells[memory_pointer] += instruction.change_in_value;
        } else {
            memory_cells[memory_pointer] -= instruction.change_in_value;
        };

        instruction_pointer += 1;
    }
}
