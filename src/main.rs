use rusty_calc::get_user_input;
use rusty_calc::lexer::{self, Token, TokenError, Value};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let mut currnt_row = 1;

    // 対話を開始
    loop {
        // ユーザーからの入力を取得
        let mut input_tokens: Vec<Token> = Vec::new();

        match get_user_input(&mut currnt_row) {
            Ok(input_rows) => {
                let tokens = lexer::tokenize(&input_rows);
                match tokens {
                    Ok(tokens) => input_tokens.extend(tokens),
                    Err(err) => match err {
                        TokenError::ExitTokenIncluded => {
                            println!("Process will be finished");
                            process::exit(0);
                        }
                        _ => {
                            continue;
                        }
                    },
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        }

        // トークンを評価
        let val = lexer::evaluate_tokens(input_tokens);
        match val {
            Ok(Value::Int(x)) => println!("- : int = {x}"),
            Ok(Value::Float(x)) => println!("- : float = {:?}", x),
            Ok(Value::Bool(x)) => println!("- : bool = {x}"),
            _ => {
                eprintln!("Unexpected evaluated value");
                process::exit(1);
            },
        }
    }
}
