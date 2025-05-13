use std::io;

fn main() {
    println!("Угадайте номер!");

    println!("Пожалуйста, введите свое предположение.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Вы угадали: {guess}");
}
