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

}
