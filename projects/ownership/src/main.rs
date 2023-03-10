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

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}", s2, len);

    // rerferences and borrowing

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    //valid multiple mutable references in curly braces

    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    //mutable and immutable references
    //will not work
    // let r1 = &s; // ok
    // let r2 = &s; // ok
    // let r3 = &mut s; // big problem

    // println!("{}, {}, and {}", r1, r2 ,r3);

    // Will work, because it changes, after we used the previous references
    // The scopes do not overlap
    let r1 = &s; // ok
    let r2 = &s; // ok
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    // Dangling references
    let reference_to_nothing = dangle();

    // The slice type

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // If the range starts from 0 we do not need to give the start index
    let slice = &s[0..2];
    let slice = &s[..2];
    // If the range ends at the last element we do not need the last index
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    //if it's go from 0 to end
    let slice = &s[..];

    let mut s = String::from("Hello world");
    let world = first_word(&s);

    // s.clear(); will drop an error to world variable, because the s variable is different, and have a reference to it.

    println!("The first world is: {}", world);
}

// With slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Without slices
fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

//do not return reference, just return the value
fn dangle() -> String {
    let s = String::from("Hello");

    s
}

//will not work because of dangling reference
// fn dangle() -> &String {
//     let s = String::from("Hello");

//     &s
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
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

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }
