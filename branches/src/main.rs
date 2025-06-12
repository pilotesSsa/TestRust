fn example_1(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn example_2(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn example_3(mut count: i32) {
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn example_4(mut number: i32) {
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn example_5() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn example_6() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn example_7() {
    fn main() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}

fn fibanachi_n(n: i32) -> i32 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else if n == 3 {
        return 1;
    }
    let mut id: i32 = 4;
    let mut fib_prev = 1;
    let mut fib_summ = 1;
    while id < n + 1 {
        let tmp = fib_summ;
        fib_summ = fib_summ + fib_prev;
        fib_prev = tmp;
        id += 1;
    }
    return fib_summ;
}

fn main() {
    //section IF
    let number = 3;
    example_1(number);

    let number = 6;
    example_2(number);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; - wrong, bcs string != int
    println!("The value of number is: {number}");

    //Section LOOP
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let count = 0;
    example_3(count);
    println!("End count = {count}");

    //Section WHILE
    let number = 3;
    example_4(number);
    example_5();
    example_6();
    example_7();

    //Conclusion
    println!("{}", fibanachi_n(10));
}
