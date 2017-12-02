use std::char;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

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
            '[' => instructions.push(Instruction::Open(
                find_matching_paren('[', ']', i + 1, ops.len()),
            )),
            ']' => instructions.push(Instruction::Close(find_matching_paren(']', '[', i - 1, 0))),
            _ => {}
        };
    }
    instructions
}

const MEMORY_SIZE: usize = 30_000;

pub fn run(source: &str, input: &str) -> String {
    let instructions = parse(source);
    let mut input_iter = input.chars();

    let mut memory = [0u8; MEMORY_SIZE];

    let mut instruction_counter: usize = 0;
    let mut memory_counter: usize = 0;

    let mut output = String::new();

    while let Some(instruction) = instructions.get(instruction_counter) {
        match *instruction {
            Instruction::MoveRight => if memory_counter + 1 == MEMORY_SIZE {
                memory_counter = 0;
            } else {
                memory_counter += 1;
            },
            Instruction::MoveLeft => if memory_counter == 0 {
                memory_counter = MEMORY_SIZE - 1;
            } else {
                memory_counter -= 1;
            },
            Instruction::Increment => memory[memory_counter] += 1,
            Instruction::Decrement => memory[memory_counter] -= 1,
            Instruction::Output => {
                output.push(char::from_u32(memory[memory_counter] as u32).unwrap())
            }
            Instruction::Input => memory[memory_counter] = input_iter.next().unwrap_or('\0') as u8,
            Instruction::Open(index) => if memory[memory_counter] == 0 {
                instruction_counter = index;
            },

            Instruction::Close(index) => if memory[memory_counter] != 0 {
                instruction_counter = index;
            },
        }

        instruction_counter += 1;
    }

    output
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn brainfuck(source: *mut c_char, input: *mut c_char) -> *mut c_char {
    let source_str = unsafe { CStr::from_ptr(source).to_str().unwrap() };
    let input_str = unsafe { CStr::from_ptr(input).to_str().unwrap() };

    let output = run(source_str, input_str);

    CString::new(output).unwrap().into_raw()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn hello_world() {
        assert_eq!(
            run(
                "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
                "",
            ),
            "Hello World!\n"
        );
    }

    #[test]
    fn cat() {
        assert_eq!(run(",[.,]", "hello"), "hello");
    }

    #[test]
    fn quine() {
        // Written by Erik Bosman
        // https://copy.sh/brainfuck/prog/quine505.b
        let program = r">+++++>+++>+++>+++++>+++>+++>+++++>++++++>+>++>+++>++++>++++>+++>+++>+++++>+>+>++++>+++++++>+>+++++>+>+>+++++>++++++>+++>+++>++>+>+>++++>++++++>++++>++++>+++>+++++>+++>+++>++++>++>+>+>+>+>++>++>++>+>+>++>+>+>++++++>++++++>+>+>++++++>++++++>+>+>+>+++++>++++++>+>+++++>+++>+++>++++>++>+>+>++>+>+>++>++>+>+>++>++>+>+>+>+>++>+>+>+>++++>++>++>+>+++++>++++++>+++>+++>+++>+++>+++>+++>++>+>+>+>+>++>+>+>++++>+++>+++>+++>+++++>+>+++++>++++++>+>+>+>++>+++>+++>+++++++>+++>++++>+>++>+>+++++++>++++++>+>+++++>++++++>+++>+++>++>++>++>++>++>++>+>++>++>++>++>++>++>++>++>++>+>++++>++>++>++>++>++>++>++>+++++>++++++>++++>+++>+++++>++++++>++++>+++>+++>++++>+>+>+>+>+++++>+++>+++++>++++++>+++>+++>+++>++>+>+>+>++++>++++[[>>>+<<<-]<]>>>>[<<[-]<[-]+++++++[>+++++++++>++++++<<-]>-.>+>[<.<<+>>>-]>]<<<[>>+>>>>+<<<<<<-]>++[>>>+>>>>++>>++>>+>>+[<<]>-]>>>-->>-->>+>>+++>>>>+[<<]<[[-[>>+<<-]>>]>.[>>]<<[[<+>-]<<]<<]";
        assert_eq!(run(program, ""), program);
    }
}
