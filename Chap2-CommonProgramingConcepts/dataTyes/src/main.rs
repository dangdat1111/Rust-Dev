use std::usize;

fn main() {
    // scalar types in Rust (số vô hướng )
    // i8,i16,i32,i64: là số dương hoặc số âm
    let x: i8  = -5;
    println!("x:{}", x);

    // isize: là số nguyên có kích thước phụ thuộc vào kiến trúc máy tính (32-bit hoặc 64-bit)
    let z: isize = -10;
    println!("z: {}", z);
    // usize: là số nguyên có kích thước phụ thuộc vào kiến trúc máy tính (32-bit hoặc 64-bit),
    // nhưng chỉ chứa số dương
    let w: usize = 10;
    println!("w: {}", w);

    // usize: là số nguyên không dấu, có kích thước phụ thuộc vào kiến trúc máy tính (32-bit hoặc
    // 64-bit)
    let a: usize = 20;
    println!("a: {}", a);
    // u8,u16,u32,u64: là số dương
    let y: u8 = 10;
    println!("y: {}", y);

    // float
    let f: f32 = 3.14; // f32: số thực 32-bit
    let g: f64 = 2.718281828459045; // f64: số thực 64-bit
    println!("f: {}, g: {}", f, g);

    let q = 3.14; // f64 mặc định
    println!("q: {}", q);

    // boolean
    let is_active: bool = true; // Kiểu boolean
    println!("is_active: {}", is_active);
    let is_logged_in: bool = false; // Kiểu boolean
    println!("is_logged_in: {}", is_logged_in);

    // character
    let char: char = 'A'; // Kiểu ký tự
    println!("char: {}", char);

    let string = " Hello, Rust!"; // Kiểu chuỗi
    println!("string: {}", string);


    // tuple: khác kiểu dữ liệu, kích thước cố định
    let tuple: (i32, f64, char) = (42, 3.14, 'R'); // Kiểu tuple
    println!("tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // array: cùng kiểu dữ liệu, kích tthước cố định
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array2 =  [1, 2, 3, 4, 5]; // Mảng kiểu i32 với kích thước 5
    println!("array: {:?}", array);
    println!("array: {}", array[0]); // Truy cập phần tử đầu tiên của mảng
    println!("array2: {:?}", array2);


    // vector: cùng kiểu dữ liệu, kích thước thay đổi
    let vector = vec![1, 2, 3, 4, 5]; // Kiểu vector
    let name = vec!["Alice", "Bob", "Charlie"]; // Vector chuỗi
    println!("vector: {:?}", vector);
    println!("name: {:?}", name);

    println!("vector first element: {}", vector[0]); // Truy cập phần tử đầu tiên của vector
    println!("name first element:{}", name[0]); //

    let mut v: Vec<i32> = Vec::new(); // Tạo một vector rỗng
    println!("Empty vector: {:?}", v);
    v.push(10); // Thêm phần tử vào vector
    v.push(20); // Thêm phần tử vào vector
    println!("Vector after adding elements: {:?}", v);

    for number in &v {
        println!("Element in vector: {}", number); // Duyệt qua các phần tử trong vector
    }

    // Use the Rust Vec type for dynamic arrays
    let mut my_ints: Vec<i32> = Vec::new();
    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(60);
    println!("vector my_ints: {:?}", my_ints);
    println!("vector my_ints size: {:?}", my_ints.len());
    println!("vector my_ints capacity: {:?}", my_ints.capacity());


    println!("First item in my_ints: {}", my_ints[0]);
    println!("First item in my_ints: {}", my_ints[1]);
    println!("First item in my_ints: {}", my_ints[2]);
    println!("first item in my_ints: {}", my_ints[3]);
    //println!("First item in my_ints: {:?}", my_ints[0..3]);

    println!("slice: {:?}", &(&my_ints).as_slice()[1..3]); // Slice of the vector
    println!("first intem in my_ints: {:?}", my_ints.get(1)); // Lấy phần tử đầu tiên của
    // vector














}
