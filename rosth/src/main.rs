use std::fs;

fn main() {
    let mut stack: Vec<i32> = Vec::new();
    let mut args = std::env::args();
    for op in fs::read_to_string(args.nth(1).expect("File path must be provided"))
        .unwrap()
        .split([' ', '\n'])
    {
        // println!("{op:?}");
        match op {
            "" => {}
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b);
            }
            "." => {
                println!("{}", stack.pop().unwrap())
            }
            _ => {
                stack.push(op.parse().unwrap());
            }
        }
    }
}
