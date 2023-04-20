use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        // 文字列型の変数のインスタンスを生成
        // mut をつけることでmutable、可変の値にすることができる
        // rustではデフォルトで定数
        let mut guess = String::new();

        io::stdin() // ユーザーからの入力を受け取る
            .read_line(&mut guess)
            .expect("Failed to read line"); // 例外処理

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
