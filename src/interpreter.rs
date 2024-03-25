use crate::parser::{Instruction, Operation};
use std::{io, io::Read};

pub fn interpret(program: Vec<Instruction>) {
    let mut memory_cells = [0; u16::MAX as usize];
    let mut memory_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;
    let mut incrementing = true;
    let mut return_address_stack: Vec<usize> = vec![];
    let mut increment_state_stack: Vec<bool> = vec![];

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
                    return_address_stack.push(instruction_pointer);
                    increment_state_stack.push(incrementing);
                }
                Operation::EndLoop => {
                    if memory_cells[memory_pointer] != 0 {
                        instruction_pointer = return_address_stack.last().unwrap().clone();
                        incrementing = increment_state_stack.last().unwrap().clone();
                    } else {
                        return_address_stack.pop();
                        increment_state_stack.pop();
                    }
                }
                Operation::Output => {
                    print!("{}", char::from_u32(memory_cells[memory_pointer]).unwrap());
                }
                Operation::Input => {
                    let value: u32 = io::stdin().bytes().next().unwrap().unwrap().into();
                    memory_cells[memory_pointer] = value;
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
