fn main() {
    {
        // s is not valid here; it's not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");

    s.push_str(" world!"); // push_str() appends a literal to a String

    println!("{}", s);

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {} ", s1, s2);
    let s3 = s1;
    // println!("{} , world!", s1);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let x1 = String::from("hello"); // x1 comes into scope
    takes_ownership(x1); // x1's value moves into the function...
                         // println!("{}", x1);     // ... and so is no longer valid here
    let x2 = 5; // x2 comes into scope
    make_copy(x2); // x2 would move into the function,
                   // but i32 is Copy, so it's okay to still use x2 afterward
    println!("{}", x2);

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);
    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3); // s2 is moved into
                        // takes_and_gives_back, which also
                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("world"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
