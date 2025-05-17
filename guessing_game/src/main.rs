use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадайте номер!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Пожалуйста, введите свое предположение:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} не число!", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое! Попытайтесь ещё раз:"),
            Ordering::Greater => println!("Слишком большое! Попытайтесь ещё раз:"),
            Ordering::Equal => {
                println!("Ты угадал(а)!");
                break;
            }
        }
    }
}
