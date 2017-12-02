use std::char;

enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Open(usize),
    Close(usize),
}

fn parse(source: &str) -> Vec<Instruction> {
    let ops: Vec<char> = source
        .chars()
        .filter(|c| match *c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        })
        .collect();

    let mut instructions = vec![];

    let find_matching_paren = |open, close, start_index, stop_index| {
        let mut index = start_index;
        let mut open_parens = 1;

        while index != stop_index {
            if ops[index] == open {
                open_parens += 1;
            } else if ops[index] == close {
                open_parens -= 1;
            }

            if open_parens == 0 {
                return index;
            }

            if start_index < stop_index {
                index += 1;
            } else {
                index -= 1;
            }
        }
        panic!("Unmatched parens");
    };
    for i in 0..ops.len() {
        match ops[i] {
            '>' => instructions.push(Instruction::MoveRight),
            '<' => instructions.push(Instruction::MoveLeft),
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => {
                instructions.push(Instruction::Open(
                    find_matching_paren('[', ']', i + 1, ops.len()),
                ))
            }
            ']' => instructions.push(Instruction::Close(find_matching_paren(']', '[', i - 1, 0))),
            _ => {}
        };
    }
    instructions


}

const MEMORY_SIZE: usize = 30000;

pub fn run(source: &str, input: &str) -> String {
    let instructions = parse(source);
    let mut input_iter = input.chars();

    let mut memory = [0u8; MEMORY_SIZE];

    let mut instruction_counter: usize = 0;
    let mut memory_counter: usize = 0;

    let mut output = String::new();

    while let Some(instruction) = instructions.get(instruction_counter) {
        match *instruction {
            Instruction::MoveRight => {
                if memory_counter + 1 == MEMORY_SIZE {
                    memory_counter = 0;
                } else {
                    memory_counter += 1;
                }
            }
            Instruction::MoveLeft => {
                if memory_counter == 0 {
                    memory_counter = MEMORY_SIZE - 1;
                } else {
                    memory_counter -= 1;
                }
            }
            Instruction::Increment => memory[memory_counter] += 1,
            Instruction::Decrement => memory[memory_counter] -= 1,
            Instruction::Output => {
                output.push(char::from_u32(memory[memory_counter] as u32).unwrap())
            }
            Instruction::Input => memory[memory_counter] = input_iter.next().unwrap() as u8,
            Instruction::Open(index) => {
                if memory[memory_counter] == 0 {
                    instruction_counter = index;
                }
            }

            Instruction::Close(index) => {
                if memory[memory_counter] != 0 {
                    instruction_counter = index;
                }
            }
        }

        instruction_counter += 1;
    }

    output
}
