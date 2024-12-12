fn main() {
    let mut x = 5;
    println!("Value of x is: {}", x);
    x = 6;
    println!("Value of x is: {}", x);
    //shadowing
    let y = 10;
    let y = y + 1;
    let y = y * 2;
    println!("Value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("number of spaces is : {}", spaces);

    //floating points
    let dec = 2.0; //f64
    let dec1: f32 = 5.0; //f32

    //Numercial operators
    let addition = 5 + 10;
    println!("sum is : {}", addition);
    let sub = 10 - 5;
    println!("sub is : {}", sub);
    let mul = 10 * 10;
    println!("mul is : {}", mul);
    let div = 25 / 5;
    println!("div is : {}", div);
    let rem = 43 % 5;
    println!("rem is : {}", rem);

    //boolean
    let is_number = true;
    let is_boolean: bool = false; // with explicit annotation

    //character
    let c = 'z';
    let z = 'Ƶ';
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat : {}", heart_eyed_cat);

    //tuples
    let tup: (i32, f64, u8) = (-500, 3.5, 8);
    println!("folating nunmber in tup is {}", tup.1);

    let (a, b, c) = tup;

    println!("a : {} , b : {}, c : {}", a, b, c);

    //Array
    let arr = [1, 10, 20, 30, 50];
    let index = 10;
    println!("1st value: {}", arr[0]);
    println!("2nd value: {}", arr[1]);
    // println!("2nd value: {}", arr[10]);
    println!("index value: {}", arr[index]);
}
