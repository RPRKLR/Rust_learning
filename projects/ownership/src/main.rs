fn main() {
    let _s = "Hello";
    let mut string = String::from("Hello");

    string.push_str(", world!"); 

    println!("{}", string);

    // Variables and Data Interacting with Move
    let x = 5;
    let y = x;

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, world!", s2);

    //deep copy
    let s1 = String::from("Hellow");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // stack only data copy
    let x = 5;
    let y = x;

    // ownership and functions
    let s = String::from("Hello");

    takes_ownership(s);

    let x = 5;

    make_copy(x);

    // return values and scope

    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);

    // returning ownership of parameters 

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}