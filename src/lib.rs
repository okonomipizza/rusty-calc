use core::str;
use std::io::{self, Error, Stdin, Write};

// 最後の文字が.になるまで入力を受け続ける
// 入力はvec[]として保持していく。
// 得られたvec[]は行ごとになるので有効なtokenに変換する
pub fn get_user_input() -> Result<Vec<String>, Error> {
    // 行番号を表示
    print!(">");
    io::stdout().flush().unwrap();

    let mut inputs: Vec<String> = Vec::new();

    // . + Enter が入力されるまで入力を受け付ける
    let mut input_continue = true;
    while input_continue {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if is_final_input(&input) {
                    input_continue = false;
                }

                inputs.push(input);
            }
            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }
    }

    Ok(inputs)
}

// 与えられた行の最後の文字が.かを判断
fn is_final_input(input: &str) -> bool {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return false;
    }
    trimmed_input.ends_with('.')
}

#[test]
fn test_is_final_input_ends_with_period() {
    assert_eq!(is_final_input("Hello world."), true);
}

#[test]
fn test_is_final_input_with_trailing_whitespace() {
    assert_eq!(is_final_input("Hello world.  "), true);
}

#[test]
fn test_is_final_input_no_period() {
    assert_eq!(is_final_input("Hello world"), false);
}

#[test]
fn test_is_final_input_empty_string() {
    assert_eq!(is_final_input(""), false)
}

#[test]
fn test_is_final_input_with_only_whitespace() {
    assert_eq!(is_final_input("   "), false)
}

#[test]
fn test_is_final_input_with_newline() {
    assert_eq!(is_final_input("Hello world.\n"), true);
}

#[test]
fn test_is_final_input_with_multiple_lines_no_period() {
    assert_eq!(is_final_input("Hello\nworld"), false)
}

#[test]
fn test_is_final_input_with_multiple_lines_ends_with_period() {
    assert_eq!(is_final_input("Hello\nworld."), true)
}
