//Là một kiểu dữ liệu tổ hợp (Custom Data Type) thuần túy, giống như một tuple có tên các trường rõ ràng hơn.
// KHÔNG hỗ trợ kế thừa (Inheritance).
// Hành vi được thêm vào thông qua impl block (triển khai), tách biệt với khai báo struct.
// Interface: Sử dụng Trait để định nghĩa hành vi chung mà nhiều struct có thể triển khai.
// Dữ liệu được quản lý bởi hệ thống Sở hữu và Mượn (Borrowing System).
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct Rectangle{
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut  user1 = User {
        active : true,
        username : String::from("dangdat1111"),
        email : String::from("dangdat9999@gmail.com"),
        sign_in_count : 1,
    };

    user1.email = String::from("datpro123@gmail.com");

    let mut user2 = User {
        email : String::from("hi@email.com"),
        active: false,
        username: String::from("hi"),
        sign_in_count: 1,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("dat dep zzaaaaa")

 
}
