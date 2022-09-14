use std::{collections::HashMap, fs};

const VERBOSE: bool = false;

fn op_pop(stack: &mut Vec<u32>, op: &str, c: u32) -> u32 {
    stack.pop().expect(&format!(
        "`{op}` operator can not be usen while there is less then {c} item in stack",
    ))
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).expect("File path must be provided"))
        .expect("Reading file failed");
    let program = file.split([' ', '\t', '\n']).collect::<Vec<&str>>();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut block_stack: Vec<usize> = Vec::new();

    for i in 0..program.len() {
        let op = program[i];
        match op {
            "if" => block_stack.push(i),
            "else" => {
                map.insert(
                    block_stack
                        .pop()
                        .expect("`else` can not be callen without `if`"),
                    i,
                );
                block_stack.push(i);
            }
            "while" | "do" => block_stack.push(i),
            "done" => {
                let a = block_stack
                    .pop()
                    .expect("`done` can not be callen without `while` and `do`");

                let b = block_stack
                    .pop()
                    .expect("`done` can not be callen without `while` and `do`");

                assert!(
                    program[a] == "do" && program[b] == "while",
                    "Only `while` and `do` can be used with `done`"
                );
                map.insert(i, b);
                map.insert(a, i);
            }

            "end" => {
                let a = block_stack
                    .pop()
                    .expect("`end` can not be callen without `if`");

                assert!(program[a] == "if", "Only `if` can be closen by `end`");
                map.insert(a, i);
            }
            _ => {}
        }
    }
    drop(block_stack);
    if VERBOSE {
        println!("map: {map:?}")
    }

    let mut i = 0;
    let mut stack: Vec<u32> = Vec::new();
    while i < program.len() {
        let op = program[i];
        if VERBOSE {
            println!("op: {op:?}")
        }
        match op {
            "" | "end" | "while" => {}
            "do" => {
                if op_pop(&mut stack, op, 1) == 0 {
                    i = *map.get(&i).unwrap();
                }
            }
            "done" => {
                i = *map.get(&i).unwrap();
            }

            "dup" => {
                let a = op_pop(&mut stack, op, 1);
                stack.push(a);
                stack.push(a);
            }
            "if" => {
                if op_pop(&mut stack, op, 1) == 0 {
                    i = *map.get(&i).unwrap();
                }
            }

            "else" => {
                i = *map.get(&i).unwrap();
            }
            "+" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push(a + b)
            }
            "-" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push(a - b)
            }
            "." => {
                println!("{}", op_pop(&mut stack, op, 1))
            }
            "=" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a == b) as u32)
            }
            "!=" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a != b) as u32)
            }
            "<" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a < b) as u32)
            }
            ">" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a > b) as u32)
            }
            "<=" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a <= b) as u32)
            }
            ">=" => {
                let a = op_pop(&mut stack, op, 2);
                let b = op_pop(&mut stack, op, 2);
                stack.push((a >= b) as u32)
            }
            _ => stack.push(op.parse().expect(&format!("`{op}` is unknown operator"))),
        }
        i += 1;
    }
}
