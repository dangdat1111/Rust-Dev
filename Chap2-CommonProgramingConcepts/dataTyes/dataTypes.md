Trong Rust, các kiểu dữ liệu được chia thành hai loại chính: **kiểu vô hướng (scalar types)** và **kiểu hợp thành (compound types)**. Sự phân loại này giúp Rust hiểu cách lưu trữ và thao tác dữ liệu một cách hiệu quả và an toàn.

-----

## Kiểu Vô Hướng (Scalar Types)

Các kiểu vô hướng đại diện cho một giá trị đơn lẻ. Chúng là những khối xây dựng cơ bản nhất của dữ liệu trong Rust.

### Các loại Kiểu Vô Hướng phổ biến:

  * **Số nguyên (Integers):**

      * **Số nguyên có dấu (Signed Integers):** `i8`, `i16`, `i32`, `i64`, `i128`, `isize`. Các số này có thể là số dương hoặc số âm. Kích thước (ví dụ: `i32` là 32-bit) xác định phạm vi giá trị mà chúng có thể chứa. `isize` và `usize` có kích thước phụ thuộc vào kiến trúc của máy tính (32-bit hoặc 64-bit).
      * **Số nguyên không dấu (Unsigned Integers):** `u8`, `u16`, `u32`, `u64`, `u128`, `usize`. Các số này chỉ có thể là số không âm.
      * **Mặc định:** Nếu bạn không chỉ định kiểu cụ thể, Rust sẽ mặc định là `i32` cho số nguyên.

  * **Số dấu phẩy động (Floating-Point Numbers):**

      * `f32`: Số dấu phẩy động 32-bit (độ chính xác đơn).
      * `f64`: Số dấu phẩy động 64-bit (độ chính xác kép). Đây là kiểu mặc định cho số dấu phẩy động.

  * **Boolean (Booleans):**

      * `bool`: Chỉ có hai giá trị `true` hoặc `false`.

  * **Ký tự (Characters):**

      * `char`: Biểu diễn một ký tự Unicode vô hướng. Ký tự trong Rust có kích thước 4 byte, lớn hơn so với nhiều ngôn ngữ khác vì nó hỗ trợ Unicode đầy đủ. Ký tự được đặt trong dấu nháy đơn (`'A'`).

Cả `isize` và `usize` trong Rust đều là các **kiểu số nguyên kích thước con trỏ** 🧱. Điều này có nghĩa là kích thước của chúng phụ thuộc vào kiến trúc của máy tính mà chương trình đang chạy. Cụ thể:

  * **Trên hệ thống 64-bit**, chúng là số nguyên 64-bit (tương đương `i64` và `u64`).
  * **Trên hệ thống 32-bit**, chúng là số nguyên 32-bit (tương đương `i32` và `u32`).

### 🎯 `isize`

`isize` là một kiểu số nguyên **có dấu** ➖, có nghĩa là nó có thể chứa cả giá trị dương và âm. Nó thường được dùng cho các phép tính số học hoặc khi bạn cần biểu diễn một giá trị có thể âm.

Ví dụ:

```rust
let x: isize = -10;
let y: isize = 20;

println!("Tổng của x và y là: {}", x + y);
```

-----

### 🎯 `usize`

`usize` là một kiểu số nguyên **không dấu** ➕, có nghĩa là nó chỉ có thể chứa các giá trị không âm (tức là từ 0 trở lên). `usize` là kiểu được sử dụng **phổ biến nhất** trong Rust để:

  * **Chỉ mục (index)** của các collection như mảng, vector, và slice.
  * **Kích thước** của các collection.
  * **Địa chỉ bộ nhớ** hoặc các con trỏ.

Việc sử dụng `usize` cho chỉ mục đảm bảo rằng chỉ mục sẽ không bao giờ là số âm và có thể bao phủ toàn bộ không gian bộ nhớ hợp lệ trên hệ thống đó.

Ví dụ:

```rust
let array = [1, 2, 3, 4, 5];
let index: usize = 2; // Chỉ mục phải là kiểu không dấu

// Truy cập phần tử thứ 3 của mảng (index 2)
println!("Phần tử tại chỉ mục {} là: {}", index, array[index]);
```

### 🧠 Tóm tắt

| Tính năng | `isize` | `usize` |
| :--- | :--- | :--- |
| **Dấu** | Có dấu (signed) | Không dấu (unsigned) |
| **Phạm vi**| Có thể âm hoặc dương | Chỉ có thể không âm (\>= 0) |
| **Sử dụng** | Phép tính số học, các giá trị có thể âm | Chỉ mục, kích thước, địa chỉ |
| **Kích thước**| Tương đương `i32`/`i64`| Tương đương `u32`/`u64`|

-----

## Kiểu Hợp Thành (Compound Types)

Các kiểu hợp thành có thể nhóm nhiều giá trị lại thành một kiểu duy nhất. Chúng cho phép bạn biểu diễn các cấu trúc dữ liệu phức tạp hơn.

### Các loại Kiểu Hợp Thành phổ biến:

  * **Tuple (Bộ):**

      * Tuple là một nhóm các giá trị thuộc nhiều kiểu dữ liệu khác nhau được nhóm lại thành một kiểu duy nhất.
      * Các giá trị trong một tuple không cần phải cùng kiểu.
      * Tuple có độ dài cố định: sau khi được khai báo, bạn không thể thêm hoặc bớt các phần tử.
      * Bạn truy cập các phần tử của tuple bằng chỉ số (index) bắt đầu từ 0.

    <!-- end list -->

    ```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup; // Destructuring
        println!("Giá trị của y là: {}", y); // Output: Giá trị của y là: 6.4

        let five_hundred = tup.0; // Truy cập bằng chỉ số
        println!("Giá trị đầu tiên là: {}", five_hundred); // Output: Giá trị đầu tiên là: 500
    }
    ```

  * **Array (Mảng):**

      * Mảng là một tập hợp các giá trị **cùng kiểu dữ liệu**.
      * Mảng có độ dài cố định: sau khi được khai báo, bạn không thể thay đổi kích thước của nó.
      * Bạn truy cập các phần tử của mảng bằng chỉ số (index) bắt đầu từ 0.

    <!-- end list -->

    ```rust
    fn main() {
        let a = [1, 2, 3, 4, 5]; // Mảng số nguyên có 5 phần tử
        println!("Phần tử thứ ba của mảng là: {}", a[2]); // Output: Phần tử thứ ba của mảng là: 3

        let months = ["Tháng Một", "Tháng Hai", "Tháng Ba"]; // Mảng chuỗi có 3 phần tử
    }
    ```

    **Lưu ý quan trọng về mảng:** Trong Rust, mảng có độ dài cố định. Nếu bạn cần một tập hợp các giá trị mà kích thước có thể thay đổi trong quá trình chạy chương trình, bạn nên sử dụng **`Vec<T>` (Vector)**, một kiểu dữ liệu collection do thư viện chuẩn của Rust cung cấp, chứ không phải là kiểu mảng nguyên thủy.

-----

## Tóm tắt Sự Khác Biệt

| Đặc điểm           | Kiểu Vô Hướng (Scalar Types) | Kiểu Hợp Thành (Compound Types)               |
| :----------------- | :--------------------------- | :-------------------------------------------- |
| **Giá trị** | Đại diện cho một giá trị đơn lẻ | Nhóm nhiều giá trị lại với nhau              |
| **Kích thước** | Cố định (ví dụ: `i32` luôn là 4 byte) | Thay đổi tùy thuộc vào số lượng và kiểu phần tử |
| **Tính linh hoạt** | Không thể thay đổi cấu trúc | Cho phép nhóm các giá trị để tạo cấu trúc phức tạp hơn |
| **Ví dụ** | `i32`, `f64`, `bool`, `char` | Tuples, Arrays                                |

Hiểu rõ sự khác biệt giữa kiểu vô hướng và kiểu hợp thành là rất quan trọng để viết mã Rust hiệu quả và an toàn. Việc này giúp bạn chọn đúng kiểu dữ liệu cho từng tình huống, tận dụng được tính an toàn bộ nhớ và hiệu suất của Rust.

Đúng vậy, bạn nên sử dụng **`Vec` (Vector)** thay vì mảng (`array`) khi bạn cần một tập hợp các giá trị có thể thay đổi kích thước. Dưới đây là những điểm chính về `Vec` và cách sử dụng nó trong Rust.

### `Vec` là gì?

**`Vec<T>`** là một kiểu dữ liệu **mảng có thể thay đổi kích thước**, được cung cấp bởi thư viện chuẩn của Rust. Nó còn được gọi là "vector".

  * **Có thể thay đổi kích thước:** Không giống như mảng, bạn có thể thêm hoặc bớt các phần tử trong `Vec` sau khi nó đã được tạo.
  * **Cùng kiểu dữ liệu:** Tương tự như mảng, tất cả các phần tử trong `Vec` đều phải có cùng kiểu dữ liệu. Kiểu dữ liệu này được chỉ định bởi tham số generic `T`. Ví dụ, `Vec<i32>` là một vector chứa các số nguyên 32-bit.
  * **Lưu trữ trên heap:** `Vec` lưu trữ dữ liệu của nó trên heap (vùng nhớ động), cho phép nó có thể thay đổi kích thước. Điều này khác với mảng, được lưu trữ trên stack (vùng nhớ tĩnh).

### Cách tạo và sử dụng `Vec`

Có hai cách phổ biến để tạo một vector:

1.  **Sử dụng macro `vec!`:** Đây là cách dễ nhất để tạo một vector với các giá trị ban đầu.

    ```rust
    let v = vec![1, 2, 3]; // Rust tự động suy ra kiểu là Vec<i32>
    let names = vec!["Alice", "Bob", "Charlie"]; // Kiểu là Vec<&str>
    ```

2.  **Sử dụng `Vec::new()`:** Phương thức này tạo một vector rỗng. Bạn phải khai báo kiểu dữ liệu một cách rõ ràng.

    ```rust
    let mut v: Vec<i32> = Vec::new();
    ```

-----

### Các thao tác phổ biến với `Vec`

  * **Thêm phần tử:** Sử dụng phương thức `push()` để thêm một phần tử vào cuối vector.

    ```rust
    let mut v = vec![10, 20];
    v.push(30);
    // v bây giờ là [10, 20, 30]
    ```

  * **Truy cập phần tử:** Bạn có thể truy cập các phần tử bằng cách sử dụng chỉ mục (`index`) trong dấu ngoặc vuông `[]`.

    ```rust
    let v = vec![1, 2, 3];
    let third_element = v[2]; // third_element có giá trị 3
    ```

    > **Lưu ý:** Nếu bạn truy cập một chỉ mục nằm ngoài phạm vi của vector, chương trình sẽ gặp lỗi "panic" và dừng lại. Để truy cập an toàn hơn, bạn có thể sử dụng phương thức `get()`, nó sẽ trả về `Some(&value)` nếu chỉ mục hợp lệ và `None` nếu không hợp lệ.

  * **Xóa phần tử:** Sử dụng phương thức `pop()` để xóa và trả về phần tử cuối cùng của vector.

    ```rust
    let mut v = vec![10, 20, 30];
    let last_element = v.pop(); // last_element là Some(30)
    // v bây giờ là [10, 20]
    ```

  * **Lặp qua các phần tử:**

    ```rust
    let v = vec![100, 32, 57];
    for number in &v {
        println!("{}", number);
    }
    ```

Nếu bạn muốn tìm hiểu sâu hơn về cách sử dụng Vec trong Rust, video này sẽ cung cấp một hướng dẫn chi tiết.

[Use the Rust Vec Type for Dynamically Expanding Arrays](https://www.youtube.com/watch?v=VIBbzFQcedU)
http://googleusercontent.com/youtube_content/0
