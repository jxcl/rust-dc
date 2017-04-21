use std::io;

enum InputToken {
    Number(i64),
    Command(char),
    Operator(char),
}

fn main() {
    let mut stack: Vec<i64> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let input = parse_input(&input);

        match input {
            InputToken::Number(num) => stack.push(num),
            InputToken::Command(c) => handle_command(c, &stack),
            InputToken::Operator(o) => handle_operator(o, &mut stack),
        }
    }
}

fn handle_operator(op: char, stack: &mut Vec<i64>) {
    match op {
        '+' => {
            let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.insert(0, a + b);
        },
        _ => panic!("Invalid operation!"),
    }
}

fn handle_command(comm: char, stack: &Vec<i64>) {
    match comm {
        'p' => {
            for i in stack {
                println!("{}", i);
            }
        },
        _ => (),
    }
}

fn parse_input(input: &str) -> InputToken {
    match input.trim().parse::<i64>() {
        Ok(num) => InputToken::Number(num),
        Err(_) => match input.chars().next() {
            Some('+') => InputToken::Operator('+'),
            Some('-') => InputToken::Operator('-'),
            Some('p') => InputToken::Command('p'),
            _ => panic!("Invalid input"),
        },
    }
}
