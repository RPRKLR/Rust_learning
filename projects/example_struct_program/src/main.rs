struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // // Tuple refactor
    // let rect(30,50);

    // println!(
    //     "The area of the rectangle is {} square pixel",
    //     areaTupple(rect)
    // );

    // // Struct example

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        areaRect(&rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn areaRect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn areaTupple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

