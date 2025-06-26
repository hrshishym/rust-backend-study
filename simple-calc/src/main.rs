use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }

        // 文字列をスペースで分割して数値に変換
        let tokens: Vec<&str> = line.trim().split(char::is_whitespace).collect();

        // 式の計算
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => {
                if right == 0.0 {
                    println!("Error: Division by zero");
                    continue;
                }
                left / right
            }
            _ => {
                unreachable!(); // ここには到達しないはず
            }
        };

        // 結果の表示
        println!("{} {} {} = {}", left, tokens[1], right, result);
    }
}
