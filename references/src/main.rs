fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}'  is {}", s1, len);

    change(&mut s1);

    println!("{}", s1);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    // println!("{}", r2);
    // println!("{}", r3);
    // let reference_to_nothing = dangle();

    let s = String::from("hello world");
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}



fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.
