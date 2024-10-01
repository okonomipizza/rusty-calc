use rusty_calc::get_user_input;

fn main() {
    // ユーザーからの入力を取得
    match get_user_input() {
        Ok(inputs) => {
            // Vec<String> の内容を行ごとに表示
            for (i, line) in inputs.iter().enumerate() {
                println!("Line {}: {}", i + 1, line.trim());
            }
        }
        Err(err) => {
            eprintln!("Error occurred: {}", err);
        }
    }
}

// 演算子
// +, -, *, /, mod, =, < , >, <=, >=

// 型
// int, float, String, bool

// 入力の終了はピリオド . で表現する

// 値は式として評価される
// また、実行中は行番号が各行に表示される
// 1> 3
// int: 3
// 2> "Hello"
// String : "Hello"

// まずは四則演算から
//
