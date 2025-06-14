fn main() {
    println!("1 + 1 = ??");
    println!("?? の値を入力してください：");

    let mut ans_input = String::new();

    std::io::stdin().read_line(&mut ans_input).unwrap();

    // dbg!(&ans_input);

    let ans_input = ans_input.trim().parse::<u32>().unwrap();
    // dbg!(ans_input);

    if ans_input == 1 + 1 {
        println!("正解です！");
    } else {
        println!("不正解です。");
    }

    println!("1 - 4 = ??");
    println!("?? の値を入力してください：");
    
    let mut ans_input = String::new();
    std::io::stdin().read_line(&mut ans_input).unwrap();
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    if ans_input == 1 - 4 {
        println!("正解です！");
    } else {
        println!("不正解です。");
    }

    println!("i32 が扱えるデータ範囲： {} - {}", i32::MIN, i32::MAX);
    println!("u32 が扱えるデータ範囲： {} - {}", u32::MIN, u32::MAX);
    println!("i64 が扱えるデータ範囲： {} - {}", i64::MIN, i64::MAX);
    println!("u64 が扱えるデータ範囲： {} - {}", u64::MIN, u64::MAX);
}
