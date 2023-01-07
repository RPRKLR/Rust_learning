fn main() {
    // if statement
    let number = 3; 

    if number < 5 {
        println!("Condition was true");
    }else {
        println!("Condition was false");
    }
    // multiple if
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else  {
        println!("Number is not divisible by 2, 3, 4");
    }
    // using if in lket statements
    let condition: bool = true;
    let _number = if condition {5} else {6};
    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    // multiple loops 
    let mut count = 0;
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
            remaining -=1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while 
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -=1;
    }
    
    println!("LIFTOFF!!!");

    //looping through a collection with for
    let a = [10; 5];
    for element in a {
        println!("The value is: {element}");
    }

    // range based for loops
    for number in (1..20).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}