pub mod calc;
pub mod lexer;
use core::str;
use std::io::{self, Error, Write};

/// ユーザーからの入力を受け付ける
pub fn get_user_input(current_row: &mut i32) -> Result<Vec<String>, Error> {
    let mut inputs: Vec<String> = Vec::new();

    // ;; + Enter が入力されるまで入力を受け付ける
    let mut input_continue = true;
    while input_continue {
        print!("#{}  ", current_row);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if is_final_input(&input) {
                    input_continue = false;
                }

                inputs.push(input);
                *current_row += 1;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                return Err(err);
            }
        }
    }
    Ok(inputs)
}

/// get_user_input() のヘルパー関数
/// 与えられた行が ;; で終わるか否かを判断する
fn is_final_input(input: &str) -> bool {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return false;
    }
    trimmed_input.ends_with(";;")
}

#[test]
fn test_is_final_input_ends_with_semi_colons() {
    assert!(is_final_input("Hello world;;"));
}

#[test]
fn test_is_final_input_ends_with_a_semi_colons() {
    assert!(!is_final_input("Hello world;"))
}

#[test]
fn test_is_final_input_ends_with_semi_colons_blank_semi_colons() {
    assert!(!is_final_input("Hello world; ;"));
}

#[test]
fn test_is_final_input_with_trailing_whitespace() {
    assert!(is_final_input("Hello world;;  "));
}

#[test]
fn test_is_final_input_no_semi_colons() {
    assert!(!is_final_input("Hello world"));
}

#[test]
fn test_is_final_input_empty_string() {
    assert!(!is_final_input(""))
}

#[test]
fn test_is_final_input_with_only_whitespace() {
    assert!(!is_final_input("   "))
}

#[test]
fn test_is_final_input_with_newline() {
    assert!(is_final_input("Hello world;;\n"));
}

#[test]
fn test_is_final_input_with_multiple_lines_no_period() {
    assert!(!is_final_input("Hello\nworld"))
}

#[test]
fn test_is_final_input_with_multiple_lines_ends_with_period() {
    assert!(is_final_input("Hello\nworld;;"))
}
