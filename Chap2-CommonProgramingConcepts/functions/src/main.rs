fn another_function(){
    println!("Another function called!");
}
// function parameter
fn parameter_function(x: &str) {
    println!("Parameter function called with value: {}", x);
}

// Function with return value
fn function_with_return_value(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("function main() called");
    another_function();
    parameter_function("Hello, parameter_function!");


    // statements and Expression in function Bodies
    let x = 5;
    let y =  {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // Function with return value

    let a = function_with_return_value(10);
    println!("The value returned by function_with_return_value is: {}", a);

}



