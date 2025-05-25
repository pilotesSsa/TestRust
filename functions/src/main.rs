fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    println!("Hello, world!");

    another_function();
    another_param_function(5);
    print_labeled_measurement(5, 'h');
    let_y_let_x();

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn let_y_let_x() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_param_function(x: i32) {
    println!("The value of x is: {x}");
}
