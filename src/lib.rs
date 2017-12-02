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
