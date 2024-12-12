fn main() {
    let number = 15;
    if number < 10 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, 2");
    }

    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The number is : {}", number);
}
