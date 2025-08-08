fn main() {
    // let x = immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6 ;
    println!("The value of x is: {x}");

    // let mut y = mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    y = y +1;
    println!("The value of y after increment is: {y}");

    let a = 10;
    let  a = a +1;
    let a = a * 2;
    println!("The value of a is: {a}");

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    //let mut spaces = "   ";
    //spaces = spaces.len();
    //println!("The number of spaces is: {spaces}");
}
