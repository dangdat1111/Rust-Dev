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

