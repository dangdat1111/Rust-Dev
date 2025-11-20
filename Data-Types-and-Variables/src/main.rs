

fn main() {

    let _x : u8 = 39;
    // println!("Hello, world!");
    
    let y : i8 = -39;
    // println!("{}", y);

    let z : f32 = 255.0;
    let _u : f64 = 111.99;
    // println!("{:?}", z);

    // bool variable
    let mut iamold : bool = false;
    iamold = true;
    // println!("{:?}", iamold);

    // Strings
    let my_str: &str = "A";
    // println!("{}", my_str);

    let first_name: &str = "dat dang";
    // println!("{:#?}", first_name);

    // tuple
    // unit type = Tuple rong ()
    let name: () =();
    // println!("{:#?}", name);

    // khai bao tuple
    let tup1 = (10, 3.14, true, "datdepzai");
    let tup2 : (i32, u64, f32, f64, bool, &str) = (-12, 255, 12.34, -13.77, false, "pro");
    let empty_tuple : () = ();

    // println!("tup1 = {:#?}", tup1);
    // println!("tup2 = {:?}", tup2);
    // print!("empty_tuple = {:#?}", empty_tuple);

    // Truy cập phần tử (destructuring) Có 2 cách chính 1: Destructuring bằng let ; 2: Truy cập bằng dấu chấm + chỉ số
    //======================================================================
    // 1: Destructuring bằng let
    let tup = (100, "hello", true);
    let (x,y,z) = tup;
    // println!("x = {:?}, y = {:?}, z = {:?}", x,y,z);

    let (score, name, _) = tup;
    // print!("score = {:?}, name = {:?}", score, name);

    //======================================================================
    // 2: Truy cập bằng dấu chấm + chỉ số
    let tup3 = (500, "Rust", 3.14);

    // println!("Phần tử đầu: {}", tup3.0);   // 500
    // println!("Phần tử thứ 2: {}", tup3.1); // "Rust"
    // println!("Phần tử cuối: {}", tup3.2);  // 3.14

    //======================================================================
    //Tuple làm giá trị trả về của hàm
    fn calculate_length(s : &str) -> (usize, &str){
        (s.len(), s)
    }
    fn get_user_info() -> (i32, String, bool) {
        (25, "Nguyễn Văn A".to_string(), true)
    }

    let user_info = get_user_info();
    // println!("{:?}", user_info);
    let (len, text) =  calculate_length("hello world");
    // println!("Độ dài = {}, nội dung = {}", len, text);

    // swap 2 bien voi tuple
    let mut a = 10;
    let mut b = 20;
    (a,b) = (b,a);
    println!("{}", a); //20
    println!("{}", b); //10
}
