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