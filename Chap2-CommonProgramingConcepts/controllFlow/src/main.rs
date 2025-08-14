fn main() {
    let number = 10;
    if number < 5{
        println!("condition was true");
    }else if number !=0 {
        println!("condition was true, but not the first one");
    }
    else {
        println!("condition was false");
    }

    let condition = false;
    let x = if condition { 5 } else { 6 };
    println!("The value of x is: {}", x);


    // Loop example
//     loop {
//         println!("datdepzai");
//     }

    // Loop with break example
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
        println!("Count is: {}", count);
    }

    // While loop example
    let mut num = 5;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    println!("LIFTOFF!!!");

}
