use rand::Rng;

fn main() {
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        let quiz_mode = rand::rng().random_range(1..=2);
        
        match quiz_mode {
            1 => {
                let op1 = rand::rng().random_range(1..=100);
                let op2 = rand::rng().random_range(1..=100);
                println!("{} + {} = ??", op1, op2);
                println!("?? の値を入力してください：");
        
                let mut ans_input = String::new();
        
                std::io::stdin().read_line(&mut ans_input).unwrap();
        
                // dbg!(&ans_input);
        
                let ans_input = ans_input.trim().parse::<u32>().unwrap();
                // dbg!(ans_input);
        
                if ans_input == op1 + op2 {
                    println!("正解です！");
                    num_of_correct += 1;
                } else {
                    println!("不正解です。");
                }
            }
            2 => {
                let op1 = rand::rng().random_range(1..=100);
                let op2 = rand::rng().random_range(1..=100);
                println!("{} - {} = ??", op1, op2);
                println!("?? の値を入力してください：");
                
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                if ans_input == op1 - op2 {
                    println!("正解です！");
                    num_of_correct += 1;
                } else {
                    println!("不正解です。");
                }
            }
            _ => {
                println!("無効なモードです。");
                continue;
            }
        }

    }

    println!("3問正解しました！おめでとうございます！");
}
