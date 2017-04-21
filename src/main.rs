use std::io;

enum InputToken {
    Number(i64),
    Commands(Vec<char>),
}

fn main() {
    let mut stack: Vec<i64> = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let words = input.split_whitespace();

        for w in words {
            let input = parse_input(w);

            match input {
                InputToken::Number(num) => stack.push(num),
                InputToken::Commands(v) => handle_commands(v, &mut stack),
            }
        }
    }
}

fn handle_commands(comms: Vec<char>, stack: &mut Vec<i64>) {
    for command in comms {
        match command {
            'p' => {
                println!("{}", stack[stack.len() - 1]);
            },
            'n' => {
                println!("{}", stack.pop().unwrap());
            },
            'f' => {
                for i in stack.iter() {
                    println!("{}", i);
                }
            },
            '+' => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a + b);
            },
            '-' => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(b - a);
            },
            '*' => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a * b);
            },
            n => panic!("Invalid operator {}!", n),
        }
    }
}

fn parse_input(input: &str) -> InputToken {
    let input = input.trim();
    let parse_result = input.parse::<i64>();

    if let Ok(num) = parse_result {
        return InputToken::Number(num);
    }

    InputToken::Commands(input.chars().collect())
}
