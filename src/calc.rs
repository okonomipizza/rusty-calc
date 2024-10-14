use core::fmt;
use std::vec;

use crate::lexer::{Operator, Token, Value};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    UnmatchedParenthesis(usize),
    InvalidTokenPattern,
    InvalidSyntax,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::UnmatchedParenthesis(index) => {
                write!(f, "Unmatched parenthesis at position {}", index)
            }
            CalcError::InvalidTokenPattern => {
                write!(f, "Invalid Tokens were suppulied")
            }
            CalcError::InvalidSyntax => {
                write!(f, "Invalid Syntax were suppulied")
            }
        }
    }
}

impl std::error::Error for CalcError {}

pub fn calc(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    // token列にintとfloatが混在している場合は、intをfloatに変換してからパターンマッチ
    let contains_int = tokens
        .iter()
        .any(|token| matches!(token, Token::Value(Value::Int(_))));
    let contains_float = tokens
        .iter()
        .any(|token| matches!(token, Token::Value(Value::Float(_))));
    let converted_tokens: Vec<Token> = if contains_int && contains_float {
        tokens
            .into_iter()
            .map(|token| match token {
                Token::Value(Value::Int(x)) => Token::Value(Value::Float(x as f64)),
                other => other,
            })
            .collect()
    } else {
        tokens
    };

    match &converted_tokens[..] {
        // 正または負の整数か実数
        [Token::Value(Value::Int(x))] => Ok(vec![Token::Value(Value::Int(*x))]),
        [Token::Value(Value::Float(x))] => Ok(vec![Token::Value(Value::Float(*x))]),
        [Token::Operator(Operator::Subtract), Token::Value(Value::Int(x))] => {
            Ok(vec![Token::Value(Value::Int(-x))])
        }
        [Token::Operator(Operator::Subtract), Token::Value(Value::Float(x))] => {
            Ok(vec![Token::Value(Value::Float(-x))])
        }

        // int同士の計算
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Add), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Int(x + y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Subtract), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Int(x - y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Multiply), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Int(x * y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Divide), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Int(x / y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Modulo), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Int(x % y))])
        }

        // float同士の計算
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Add), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Float(x + y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Subtract), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Float(x - y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Multiply), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Float(x * y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Divide), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Float(x / y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Modulo), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Float(x % y))])
        }

        // integers ==, <=, >=, <, >
        [Token::Value(Value::Int(x)), Token::Operator(Operator::Equal), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Bool(x == y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::LessThan), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Bool(x < y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::GreaterThan), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Bool(x > y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::GreaterEqual), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Bool(x >= y))])
        }
        [Token::Value(Value::Int(x)), Token::Operator(Operator::LessEqual), Token::Value(Value::Int(y))] => {
            Ok(vec![Token::Value(Value::Bool(x <= y))])
        }

        // floats ==, <=, >=, <, >
        [Token::Value(Value::Float(x)), Token::Operator(Operator::Equal), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Bool(x == y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::LessThan), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Bool(x < y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::GreaterThan), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Bool(x > y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::GreaterEqual), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Bool(x >= y))])
        }
        [Token::Value(Value::Float(x)), Token::Operator(Operator::LessEqual), Token::Value(Value::Float(y))] => {
            Ok(vec![Token::Value(Value::Bool(x <= y))])
        }

        _ => {
            // ()を評価する
            // 括弧の位置と数を調べて、calcを呼び出す必要がある
            let mut left_parenthesis_positions: Vec<usize> = Vec::new();
            let mut parenthesis_positions: Vec<(usize, usize)> = Vec::new();

            // left_parenthesis_positionsにFILOで入れていく
            for (i, token) in converted_tokens.iter().enumerate() {
                if *token == Token::Operator(Operator::LeftParenthesis) {
                    left_parenthesis_positions.push(i);
                } else if *token == Token::Operator(Operator::RightParenthesis) {
                    match left_parenthesis_positions.pop() {
                        Some(left_index) => parenthesis_positions.push((left_index, i)),
                        None => {
                            return Err(CalcError::UnmatchedParenthesis(i));
                        }
                    }
                }
            }

            let mut new_tokens: Vec<Token> = converted_tokens.clone().to_vec();
            let mut shorter_flag: usize = 0; // ()の中の式を評価するとそれ以降の式の index が変わってしまうので補正する必要がある
            for parenthesis in parenthesis_positions {
                let start_index = parenthesis.0 - shorter_flag;
                let end_index = parenthesis.1 - shorter_flag;

                // ()の中の式を再帰的に計算
                match calc(new_tokens[start_index + 1..end_index].to_vec()) {
                    Ok(result_tokens) => {
                        new_tokens[start_index] = result_tokens[0].clone(); // '('を得られた値に置き換える
                        new_tokens.drain(start_index + 1..=end_index); // ')'までは不要なので捨てる
                        shorter_flag += end_index - start_index
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            // *, /, %を評価する
            let mut mul_div_mod_indices: Vec<usize> = Vec::new();
            for (i, token) in new_tokens.iter().enumerate() {
                if let Token::Operator(Operator::Multiply | Operator::Divide | Operator::Modulo) =
                    token
                {
                    mul_div_mod_indices.push(i);
                }
            }
            for &index in mul_div_mod_indices.iter().rev() {
                // index の範囲を確認
                if index == 0 || index + 1 >= new_tokens.len() {
                    return Err(CalcError::InvalidSyntax);
                };

                let start_index = index - 1;
                let end_index = index + 1;

                // スライスの範囲を取り出して計算
                match calc(new_tokens[start_index..=end_index].to_vec()) {
                    Ok(result_tokens) => {
                        new_tokens[start_index] = result_tokens[0].clone();
                        new_tokens.drain(index..=end_index);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            // +, -を評価する
            let mut add_sub_indices: Vec<usize> = Vec::new();
            for (i, token) in new_tokens.iter().enumerate() {
                if let Token::Operator(Operator::Add | Operator::Subtract) = token {
                    add_sub_indices.push(i);
                }
            }
            for &index in add_sub_indices.iter().rev() {
                // index の範囲を確認
                if index == 0 || index + 1 >= new_tokens.len() {
                    return Err(CalcError::InvalidSyntax);
                };

                let start_index = index - 1;
                let end_index = index + 1;

                // スライスの範囲を取り出して計算
                match calc(new_tokens[start_index..=end_index].to_vec()) {
                    Ok(result_tokens) => {
                        new_tokens[start_index] = result_tokens[0].clone();
                        new_tokens.drain(index..=end_index);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            // == , >=, <=, >, <を評価する
            let mut compare_indices: Vec<usize> = Vec::new();
            for (i, token) in new_tokens.iter().enumerate() {
                if let Token::Operator(
                    Operator::Equal
                    | Operator::GreaterEqual
                    | Operator::GreaterThan
                    | Operator::LessEqual
                    | Operator::LessThan,
                ) = token
                {
                    compare_indices.push(i);
                };
            }
            for &index in compare_indices.iter().rev() {
                if index == 0 || index + 1 >= new_tokens.len() {
                    return Err(CalcError::InvalidSyntax);
                };

                let start_index = index - 1;
                let end_index = index + 1;
                // スライスの範囲を取り出して計算
                match calc(new_tokens[start_index..=end_index].to_vec()) {
                    Ok(result_tokens) => {
                        new_tokens[start_index] = result_tokens[0].clone();
                        new_tokens.drain(index..=end_index);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Ok(new_tokens)
        }
    }
}

#[cfg(test)]
fn assert_calc_result(tokens: Vec<Token>, expected: Result<Value, CalcError>) {
    let result = calc(tokens);
    match (result, expected) {
        (Ok(new_tokens), Ok(expected_value)) => {
            assert_eq!(new_tokens.len(), 1); // 式の評価結果は単一の Token になるはず
            match &new_tokens[0] {
                Token::Value(value) => assert_eq!(value, &expected_value),
                _ => panic!("Unexpected token type"),
            }
        }
        (Err(err), Err(expected_err)) => {
            assert_eq!(err, expected_err);
        }
        (Ok(_), Err(_)) => {
            panic!("Expected an error but got a successful result.");
        }
        (Err(_), Ok(_)) => {
            panic!("Expected a successful result but got an error.");
        }
    }
}

#[test]
fn test_calc_single_integer() {
    // 5 -> 5
    let tokens = vec![Token::Value(Value::Int(5))];
    assert_calc_result(tokens, Ok(Value::Int(5)));
}

#[test]
fn test_calc_single_float() {
    // 1.23 -> 1.23
    let tokens = vec![Token::Value(Value::Float(1.23))];
    assert_calc_result(tokens, Ok(Value::Float(1.23)));
}

#[test]
fn test_calc_sum_of_2_integers() {
    // 2 + 3 -> 5
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(5)));
}

#[test]
fn test_calc_sum_of_integer_float() {
    // 2.3 + 3 -> 5.3
    let tokens = vec![
        Token::Value(Value::Float(2.3)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Float(5.3)));
}

#[test]
fn test_calc_sum_of_2_floats() {
    // 1.2 + 2.3 -> 3.5
    let tokens = vec![
        Token::Value(Value::Float(1.2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Float(2.3)),
    ];
    assert_calc_result(tokens, Ok(Value::Float(3.5)));
}

#[test]
fn test_calc_sum_of_3_integers() {
    // 1 + 2 + 3 -> 6
    let tokens = vec![
        Token::Value(Value::Int(1)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(6)));
}

#[test]
fn test_calc_sum_of_3_floats() {
    // 1.2 + 2.3 + 3.4 -> 6.9
    let tokens = vec![
        Token::Value(Value::Float(1.2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Float(2.3)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Float(3.4)),
    ];

    let expected_result = 1.2 + 2.3 + 3.4;
    let result = calc(tokens);

    match result {
        Ok(new_tokens) => {
            assert_eq!(new_tokens.len(), 1);
            if let Token::Value(Value::Float(value)) = new_tokens[0] {
                assert!(
                    (value - expected_result).abs() < 0.001,
                    "Expected {}, got {}",
                    expected_result,
                    value
                );
            } else {
                panic!("Unexpected token type");
            }
        }
        Err(err) => panic!("calc failed with error: {:?}", err),
    }
}

#[test]
fn test_calc_multiply_integer_by_integers() {
    // 2 * 3 -> 6
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Multiply),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(6)));
}

#[test]
fn test_calc_multiply_float_by_float() {
    // 2.0 * 3.0 -> 6.0
    let tokens = vec![
        Token::Value(Value::Float(2.0)),
        Token::Operator(Operator::Multiply),
        Token::Value(Value::Float(3.0)),
    ];
    assert_calc_result(tokens, Ok(Value::Float(6.0)));
}

#[test]
fn test_calc_divide_integer_by_integer() {
    // 6 / 3 -> 2
    let tokens = vec![
        Token::Value(Value::Int(6)),
        Token::Operator(Operator::Divide),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(2)));
}

#[test]
fn test_calc_divide_float_by_float() {
    // 3.6 / 1.2 -> 3
    let tokens = vec![
        Token::Value(Value::Float(3.6)),
        Token::Operator(Operator::Divide),
        Token::Value(Value::Float(1.2)),
    ];
    assert_calc_result(tokens, Ok(Value::Float(3.0)));
}

#[test]
fn test_calc_modulo_integers() {
    // 5 / 3 -> 2
    let tokens = vec![
        Token::Value(Value::Int(5)),
        Token::Operator(Operator::Modulo),
        Token::Value(Value::Int(3)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(2)));
}

#[test]
fn test_calc_modulo_floats() {
    // 5.3 / 2.5 -> 3
    let tokens = vec![
        Token::Value(Value::Float(5.3)),
        Token::Operator(Operator::Modulo),
        Token::Value(Value::Float(2.5)),
    ];
    assert_calc_result(tokens, Ok(Value::Float(5.3 % 2.5)));
}

#[test]
fn test_calc_priority_sum_mul() {
    // 2 + 3 * 6 = 20
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::Multiply),
        Token::Value(Value::Int(6)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(20)));
}

#[test]
fn test_with_tuple_1() {
    // (2 + 3) * 6 = 30
    let tokens = vec![
        Token::Operator(Operator::LeftParenthesis),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::RightParenthesis),
        Token::Operator(Operator::Multiply),
        Token::Value(Value::Int(6)),
    ];
    assert_calc_result(tokens, Ok(Value::Int(30)));
}

#[test]
fn test_with_two_tuple() {
    // (2 + 3) - (2 + 3) = 0
    let tokens = vec![
        Token::Operator(Operator::LeftParenthesis),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::RightParenthesis),
        Token::Operator(Operator::Subtract),
        Token::Operator(Operator::LeftParenthesis),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::RightParenthesis),
    ];
    assert_calc_result(tokens, Ok(Value::Int(0)));
}

#[test]
fn test_calc_priority_with_outside_tuple() {
    // (2 + 3 + 4) = 9
    let tokens = vec![
        Token::Operator(Operator::LeftParenthesis),
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(3)),
        Token::Operator(Operator::Add),
        Token::Value(Value::Int(4)),
        Token::Operator(Operator::RightParenthesis),
    ];
    assert_calc_result(tokens, Ok(Value::Int(9)));
}

#[test]
fn test_calc_equal_integers() {
    // 2 == 2 -> true
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_not_equal_integers() {
    // 2 == 5 -> false
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Int(5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(false)));
}

#[test]
fn test_calc_equal_floats() {
    // 2.5 == 2.5 -> true
    let tokens = vec![
        Token::Value(Value::Float(2.5)),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Float(2.5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_less_than_true() {
    // 2 < 5 -> true
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::LessThan),
        Token::Value(Value::Int(5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_less_than_false1() {
    // 5 < 2 -> false
    let tokens = vec![
        Token::Value(Value::Int(5)),
        Token::Operator(Operator::LessThan),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(false)));
}

#[test]
fn test_calc_less_than_false2() {
    // 2 < 2 -> false
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::LessThan),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(false)));
}

#[test]
fn test_calc_less_eq_true1() {
    // 2 <= 2 -> true
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::LessEqual),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_less_eq_true2() {
    // 2 <= 5 -> true
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::LessEqual),
        Token::Value(Value::Int(5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_grater_than() {
    // 2 > 5 -> false
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::GreaterThan),
        Token::Value(Value::Int(5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(false)));
}

#[test]
fn test_calc_grater_eq_true() {
    // 5 >= 2 -> true
    let tokens = vec![
        Token::Value(Value::Int(5)),
        Token::Operator(Operator::GreaterEqual),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_grater_eq_true2() {
    // 2 >= 2 -> true
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::GreaterEqual),
        Token::Value(Value::Int(2)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(true)));
}

#[test]
fn test_calc_grater_eq_false() {
    // 2 >= 5 -> false
    let tokens = vec![
        Token::Value(Value::Int(2)),
        Token::Operator(Operator::GreaterEqual),
        Token::Value(Value::Int(5)),
    ];
    assert_calc_result(tokens, Ok(Value::Bool(false)));
}

#[test]
fn test_calc_err_invalid_syntax() {
    // + 5 -> err
    let tokens = vec![Token::Operator(Operator::Add), Token::Value(Value::Int(5))];
    assert_calc_result(tokens, Err(CalcError::InvalidSyntax));
}

#[test]
fn test_calc_err_invalid_syntax_only_operator() {
    // + -> err
    let tokens = vec![Token::Operator(Operator::Add)];
    assert_calc_result(tokens, Err(CalcError::InvalidSyntax));
}

#[test]
fn test_calc_err_invalid_syntax_only_3_adds() {
    // + -> err
    let tokens = vec![
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Add),
    ];
    assert_calc_result(tokens, Err(CalcError::InvalidSyntax));
}
