Trong Rust, **shadowing** là khả năng khai báo một biến mới có cùng tên với một biến đã tồn tại trong phạm vi hiện tại. Điều này cho phép bạn "che khuất" (shadow) biến cũ, biến mới sẽ được sử dụng trong mọi hoạt động sau đó.

-----

### Khác biệt với Mutability

Shadowing khác với việc sử dụng từ khóa `mut`. Khi bạn sử dụng `mut`, bạn đang thay đổi giá trị của cùng một biến. Với shadowing, bạn đang tạo ra một biến **hoàn toàn mới**, cho phép bạn thay đổi cả kiểu dữ liệu của nó.

**Ví dụ về Shadowing:**

```rust
fn main() {
    let x = 5; // Biến x đầu tiên, kiểu i32

    let x = x + 1; // Tạo một biến x mới, giá trị là 6

    {
        let x = x * 2; // Tạo một biến x mới trong khối bên trong, giá trị là 12
        println!("Giá trị của x trong khối bên trong là: {}", x); // In ra 12
    }

    println!("Giá trị của x sau khối bên trong là: {}", x); // In ra 6
}
```

Trong ví dụ trên, biến `x` ban đầu được gán giá trị `5`. Sau đó, chúng ta khai báo lại `x` với giá trị `x + 1`, tạo ra một biến `x` mới với giá trị `6`. Trong một khối mã (block) bên trong, một biến `x` khác được tạo, nhưng nó chỉ tồn tại trong phạm vi của khối đó. Khi khối kết thúc, biến `x` cũ với giá trị `6` sẽ được khôi phục.

-----

### Lợi ích của Shadowing

  * **Tái sử dụng tên biến:** Shadowing rất hữu ích khi bạn muốn thực hiện một chuỗi các phép biến đổi trên một giá trị mà không cần phải đặt tên biến mới cho mỗi bước.
  * **Thay đổi kiểu dữ liệu:** Shadowing cho phép bạn thay đổi kiểu dữ liệu của biến. Ví dụ:
    ```rust
    let spaces = "   ";
    let spaces = spaces.len(); // Biến spaces mới có kiểu usize
    ```
    Đây là điều mà `mut` không thể làm được. Nếu bạn cố gắng thay đổi kiểu dữ liệu của một biến `mut`, trình biên dịch sẽ báo lỗi.

-----

### ⚠️ Lưu ý

  * Shadowing chỉ hoạt động trong phạm vi (scope) hiện tại. Khi một khối mã kết thúc, biến được "shadow" bên trong khối đó sẽ biến mất và biến ban đầu sẽ được khôi phục.
  * Sử dụng shadowing một cách hợp lý để tăng tính rõ ràng của mã, tránh lạm dụng nó có thể làm cho mã khó đọc hơn.
