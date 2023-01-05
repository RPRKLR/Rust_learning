fn main() {
    let  x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {x}");
   
    let x = x + 1;

    {
	let x = x * 2;
	println!("The value of x in the inner scope is: {x}");
    }
    println!("The value fo x is: {x}");
    // shadowing will not cause errors
    let spaces = "   ";
    let spaces = spaces.len();
    // using mut will cause errors
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // //----------- Data types -------------\\

    let guess: u32 = "42".parse().expect("Not a number");

    let x = 2.0;  // f64
    let y: f32 = 3.0 // f32

    // //----------- Numeric Operations ----------\\

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let x = 'z';
    let z: char = 'Z';
    
    // //----------- Compound types ------------\\

    //tuple 
    let tup: (i32, f64, u8) = (500,3.6,1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array
    let a = [1, 2,3,4,5];
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    // Array contains same values.
    let a = [3; 5];
    //accesing an array's element
    

}





















