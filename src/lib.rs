use std::char;

enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Open,
    Close,
}

fn parse(source: &str) -> Vec<Instruction> {
    source
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Instruction::MoveRight),
            '<' => Some(Instruction::MoveLeft),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::Open),
            ']' => Some(Instruction::Close),
            _ => None,
        })
        .collect()
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
            _ => unimplemented!(),
        }

        instruction_counter += 1;
    }

    output
}
