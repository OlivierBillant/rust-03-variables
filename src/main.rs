fn main() {
    // println!("Hello, world!");

    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const TEST_CONSTANT: u32 = 10_000;
    println!("The const value is: {}", TEST_CONSTANT);
    let y: i32 = 50;
    println!("The value of x is: {}", y);
    let y: i32 = 60;
    println!("The value of x is: {}", y);
    let y: &str = "sixty";
    println!("The value of x is: {}", y);
    let y:i32 = 60;
    println!("The sum of x and y is {}", my_function(x, y));
}

fn my_function(x: i32, y:i32)-> i32{
    println!("A new function called up in main !");
    return x+y;
}
