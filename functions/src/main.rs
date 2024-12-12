fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let x = plus_one(5);
    println!(" x value is {}", x);

    another_function();
    print_value(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("another function");
}

fn print_value(x: i32) {
    println!("The value of x: {}", x);
}
