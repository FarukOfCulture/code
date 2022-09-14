use std::fs;

fn op_pop(stack: &mut Vec<u32>, op: &str, c: u32) -> u32 {
    stack.pop().expect(&format!(
        "`{op}` operator can not be usen while there is less then {c} item in stack",
    ))
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).expect("File path must be provided"))
        .expect("Reading file failed");
    let program = file.split([' ', '\t', '\n']).collect::<Vec<&str>>();

    let mut block_stack: Vec<usize> = Vec::new();
    for i in (0..program.len()).rev() {
        let op = program[i];
        match op {
            "else" | "end" => block_stack.push(i),
            _ => {}
        }
    }

    let mut i = 0;
    let mut stack: Vec<u32> = Vec::new();
    while i < program.len() {
        let op = program[i];
        // println!("{op:?}");
        match op {
            "" => {}
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
