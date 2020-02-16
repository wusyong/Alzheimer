use async_std::io::{ReadExt, stdin};
use async_std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug)]
enum Token {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    Loop(Vec<Token>),
}

pub struct IntepreterBuilder(Vec<Token>);

impl IntepreterBuilder {
    pub async fn new(inputs: String) -> IntepreterBuilder {
        let mut token: Vec<Token> = Vec::new();

        inputs.chars().into_iter().for_each(|symbol| {
            match symbol {
                '>' => token.push(Token::IncrementPtr),
                '<' => token.push(Token::DecrementPtr),
                '+' => token.push(Token::Increment),
                '-' => token.push(Token::Decrement),
                '.' => token.push(Token::Output),
                ',' => token.push(Token::Input),
                '[' => token.push(Token::LoopStart),
                ']' => token.push(Token::LoopEnd),
                _ => (),
            } 
        });

        IntepreterBuilder(token)
    }

    pub async fn build(self) -> Intepreter {
        Intepreter(Self::parse(self.0).await)
    }

    fn parse(inputs: Vec<Token>) -> Pin<Box<dyn Future<Output = Vec<Token>>>> {
        Box::pin(async move {
            let mut token: Vec<Token> = Vec::new();
            let mut stack = 0;
            let mut start = 0;
        
            for (i, c) in inputs.iter().enumerate() {
                if stack == 0 {
                    match c {
                        Token::IncrementPtr => token.push(Token::IncrementPtr),
                        Token::DecrementPtr => token.push(Token::DecrementPtr),
                        Token::Increment => token.push(Token::Increment),
                        Token::Decrement => token.push(Token::Decrement),
                        Token::Output => token.push(Token::Output),
                        Token::Input => token.push(Token::Input),
                        Token::LoopStart => {
                            start = i;
                            stack += 1;
                        },
                        Token::LoopEnd => panic!("Loop ending at position {} missing a beginning", i),
                        _ => (),
                    }
                } else {
                    match c {
                        Token::LoopStart => stack += 1,
                        Token::LoopEnd => {
                            stack -= 1;
                            if stack == 0 {
                                token.push(Token::Loop(Self::parse(inputs[start+1..i].to_vec()).await));
                            }
                        }
                        _ => (),
                    }
                }
            }
        
            if stack != 0 {
                panic!("Loop starting at position {} missing a ending", start);
            }
        
            token
        })
    }
}

pub struct Intepreter(Vec<Token>);

impl Intepreter {
    pub async fn run (self) {
        Self::exec(&self.0, &mut vec![0u8; 1024], 0).await;
    }

    fn exec<'a>(parser: &'a Vec<Token>, tape: &'a mut Vec<u8>, ptr: usize) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            let mut ptr = ptr;

            for i in parser {
                match i {
                    Token::IncrementPtr => ptr += 1,
                    Token::DecrementPtr => ptr -= 1,
                    Token::Increment => tape[ptr] += 1,
                    Token::Decrement => tape[ptr] -= 1,
                    Token::Output => print!("{}", tape[ptr] as char),
                    Token::Input => {
                        let mut input = [0u8; 1];
                        stdin().read_exact(&mut input).await.expect("Failed to read stdin");
                        tape[ptr] = input[0];
                    },
                    Token::Loop(tokens) => {
                        while tape[ptr] != 0 {
                            Self::exec(&tokens, tape, ptr).await;
                        }
                    },
                    _ => (),
                } 
            }
        })
    }
}
