fn main() {
    // stack and heap

    let  s1 = String::from("dat dep zai");
    let s2 = s1;

    println!("{s2}, world");

    // sao chép sâu dữ liệu heap của String
    // tạo 1 bản sao hoàn chỉnh của dữ liệu heap
    // => tốn bộ nhớ và thời gian hơn
    let a1 = String::from("hello");
    let a2 = a1.clone();
    println!("{a1}, world");


    // Ownership and Functions
    let s = String::from("hiiiiii");
    takes_ownership(s);
    println!("{}", s); // s đã bị move vào hàm takes_ownership

    let x = 5;
    makes_copy(x);
    println!("{}", x); // x vẫn còn giá trị vì i32 implement Copy trait
    // dữ liệu không triển khai Copy( String, Vec<T>, hoặc Struct có chứa chúng ) -> sẽ bị move
    // (không còn quyền sở hữu)
    // dữ liệu triển khai Copy( tất cả kiểu số nguyên, bool, char, float, Tuple chỉ chứa các kiểu
    // dữ liệu triển khai Copy ) -> sẽ được sao chép
    //
    // không thể vừa có bản sao vừa có quyền sở hữu
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    let a1 = String::from("hello");
    let (a2, len) = calculate_length(a1);
    println!("The length of '{a2}' is {len}.");

    let x1 = String::from("what the fuck");
    let len1 = calculate_length_reference(&x1); //x1 vẫn giữ quyền sở hữu
    println!("The length of '{x1}' is {len1}.");
}
fn takes_ownership(some_string : String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}



fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String)-> (String,usize){
    let length = s.len();
    (s,length)
}

fn calculate_length_reference(s: &String) -> usize{
    s.len()
}

// dangling reference ( con trỏ treo )





