use core::fmt;

use crate::calc::calc;

/// サポートしているデータ型
///
/// i32, f64, bool
#[derive(Debug, PartialEq, std::clone::Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
}

/// サポートしている演算子
///
/// +, -, *, /, mod, =, < , >, <=, >=, (, )
#[derive(Debug, PartialEq, std::clone::Clone)]
pub enum Operator {
    Add,              // +
    Subtract,         // -
    Multiply,         // *
    Divide,           // /
    Modulo,           // mod
    Equal,            // ==
    LessThan,         // <
    GreaterThan,      // >
    LessEqual,        // <=
    GreaterEqual,     // >=
    LeftParenthesis,  //　(
    RightParenthesis, //　)
}

/// 予約語
#[derive(Debug, PartialEq, std::clone::Clone)]
pub enum Expected {
    Semicolon, // ユーザー入力の終了
    Exit,      // プログラムの終了
}

/// トークン
///
/// 値, 演算子, 予約語のどれか
#[derive(Debug, PartialEq, std::clone::Clone)]
pub enum Token {
    Value(Value),
    Operator(Operator),
    Expected(Expected),
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidTokenPattern,
    ExitTokenIncluded,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenError::InvalidTokenPattern => {
                write!(f, "Invalid Tokens were suppulied")
            }
            TokenError::ExitTokenIncluded => {
                write!(f, "Exit was typed")
            }
        }
    }
}

/// 入力行をトークン化
pub fn tokenize(input_rows: &Vec<String>) -> Result<Vec<Token>, TokenError> {
    let mut result: Vec<Token> = Vec::new();

    for row in input_rows {
        for word in row.split_whitespace() {
            if word == ";;" {
                continue;
            }

            let token = match word {
                // 演算子
                "+" => Ok(Token::Operator(Operator::Add)),
                "-" => Ok(Token::Operator(Operator::Subtract)),
                "*" => Ok(Token::Operator(Operator::Multiply)),
                "/" => Ok(Token::Operator(Operator::Divide)),
                "mod" => Ok(Token::Operator(Operator::Modulo)),
                "==" => Ok(Token::Operator(Operator::Equal)),
                "<" => Ok(Token::Operator(Operator::LessThan)),
                ">" => Ok(Token::Operator(Operator::GreaterThan)),
                "<=" => Ok(Token::Operator(Operator::LessEqual)),
                ">=" => Ok(Token::Operator(Operator::GreaterEqual)),
                "(" => Ok(Token::Operator(Operator::LeftParenthesis)),
                ")" => Ok(Token::Operator(Operator::RightParenthesis)),
                // 予約語
                "exit" => Err(TokenError::ExitTokenIncluded),
                _ => {
                    // 値
                    if let Ok(i) = word.parse::<i32>() {
                        Ok(Token::Value(Value::Int(i)))
                    } else if let Ok(f) = word.parse::<f64>() {
                        Ok(Token::Value(Value::Float(f)))
                    } else {
                        eprintln!("Invalid token: '{}'", word);
                        Err(TokenError::InvalidTokenPattern)
                    }
                }
            };

            match token {
                Ok(t) => result.push(t),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(result)
}

/// トークンのベクタを解析して評価結果を返す
/// 結果は型: 値の形で表示される
pub fn evaluate_tokens(tokens: Vec<Token>) -> Result<Value, TokenError> {
    match &tokens[..] {
        // 値のみの評価
        [Token::Value(Value::Int(x))] => Ok(Value::Int(*x)),
        [Token::Value(Value::Float(x))] => Ok(Value::Float(*x)),
        [Token::Value(Value::Bool(x))] => Ok(Value::Bool(*x)),

        // 演算子の評価
        _ => match calc(tokens) {
            Ok(result) => evaluate_tokens(result),
            Err(_) => {
                println!("Some error !!");
                Err(TokenError::InvalidTokenPattern)
            }
        },
    }
}

#[test]
fn test_tokenize_valid_tokens() {
    let input = vec![String::from("2 + 3 * 5 / ( 1.5 - 2 )"), String::from(" ;;")];
    let expected = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::Multiply),
        Token::Value(Value::Int(5)),
        Token::Operator(Operator::Divide),
        Token::Operator(Operator::LeftParenthesis),
        Token::Value(Value::Float(1.5)),
        Token::Operator(Operator::Subtract),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::RightParenthesis),
    ];
    let result = tokenize(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_tokenize_invalid_token() {
    let input = vec![String::from("2 + a")];
    let result = tokenize(&input);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), TokenError::InvalidTokenPattern);
}

#[test]
fn test_tokenize_ignore_double_semicolon() {
    let input = vec![String::from("2 + 3 ;;")];
    let expected = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
    ];

    let result = tokenize(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_tokenize_mixed_tokens() {
    let input = vec![String::from("1 + 2.5 mod 3")];
    let expected = vec![
        Token::Value(Value::Int(1)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Float(2.5)),
        Token::Operator(Operator::Modulo),
        Token::Value(Value::Int(3)),
    ];

    let result = tokenize(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}
