fn another_function(){
    println!("Another function called!");
}
// function parameter
fn parameter_function(x: &str) {
    println!("Parameter function called with value: {}", x);
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

}



