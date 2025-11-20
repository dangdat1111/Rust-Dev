Khái niệm **Ownership (quyền sở hữu)** trong Rust là một phần **cốt lõi** của ngôn ngữ, giúp quản lý bộ nhớ **mà không cần garbage collector**. Dưới đây là phần giải thích dễ hiểu và có ví dụ minh họa.

---

## ✅ Giải thích dễ hiểu về Ownership trong Rust

### Tưởng tượng:

Bạn có một **quyển sách**. Nếu bạn **đưa quyển sách đó cho người khác**, thì bạn **không còn giữ nó nữa**. Người kia **toàn quyền sở hữu** quyển sách. Đây chính là **Ownership**.

Trong Rust:

* Mỗi **giá trị** (value) chỉ có **một owner (người sở hữu)** tại một thời điểm.
* Khi bạn **gán giá trị cho biến khác**, **quyền sở hữu được chuyển giao** (move).
* Khi **owner ra khỏi scope**, giá trị sẽ **bị giải phóng** (drop tự động).

---

## 🧠 Tại sao cần Ownership?

Rust dùng Ownership để:

* Quản lý bộ nhớ tự động và hiệu quả.
* Ngăn lỗi như **use-after-free**, **double free**, **memory leaks**.

---

## 📦 Ví dụ 1: Chuyển quyền sở hữu (move)

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s1); // ❌ Lỗi! s1 không còn sở hữu dữ liệu nữa.
}
```

### Giải thích:

* `s1` sở hữu chuỗi `"Hello"`.
* Khi gán `s2 = s1`, quyền sở hữu được chuyển sang `s2`.
* `s1` không còn hợp lệ => lỗi khi dùng `s1`.

---

## ✅ Cách fix: clone

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2); // ✅ OK
}
```

### Giải thích:

* `.clone()` tạo bản sao dữ liệu trong bộ nhớ.
* `s1` và `s2` là **hai owner độc lập**.

---

## 📦 Ví dụ 2: Ownership với hàm

```rust
fn takes_ownership(s: String) {
    println!("Got: {}", s);
} // s bị drop ở đây

fn main() {
    let s = String::from("Rust");
    takes_ownership(s);

    println!("{}", s); // ❌ Lỗi! s đã bị chuyển và drop rồi.
}
```

### Giải thích:

* Khi truyền `s` vào hàm, ownership được chuyển sang tham số `s` trong hàm.
* Biến gốc `s` không còn hợp lệ sau khi gọi hàm.

---

## ✅ Cách fix: Trả lại ownership hoặc dùng reference

### Cách 1: Trả lại ownership

```rust
fn takes_and_returns(s: String) -> String {
    s
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = takes_and_returns(s1);
    println!("{}", s2);
}
```

### Cách 2: Dùng mượn (borrow)

```rust
fn print_str(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("World");
    print_str(&s);
    println!("{}", s); // ✅ OK vì không chuyển ownership
}
```

---

## 🧩 Tổng kết 3 quy tắc Ownership

1. **Mỗi giá trị có một chủ sở hữu duy nhất.**
2. Khi gán giá trị cho biến khác, **ownership được chuyển giao.**
3. Khi owner ra khỏi phạm vi, giá trị bị **giải phóng**.

---

Nếu bạn muốn, mình có thể vẽ sơ đồ hoặc tạo thêm ví dụ nâng cao về **Borrowing**, **Mutable Borrowing**, hoặc **Ownership với Struct**.



1. **Borrowing (mượn dữ liệu)**
2. **Mutable Borrowing (mượn dữ liệu có thể thay đổi)**
3. **Ownership với Struct**

Kèm theo **sơ đồ minh họa ASCII** dễ hiểu và **ví dụ chi tiết**.

---

## 🧾 1. **Borrowing – Mượn dữ liệu (tham chiếu bất biến)**

### ✔ Cho phép hàm mượn dữ liệu mà không chiếm quyền sở hữu.

```rust
fn print_length(s: &String) {
    println!("Độ dài: {}", s.len());
}

fn main() {
    let s = String::from("Xin chào");
    print_length(&s);      // truyền tham chiếu (mượn)
    println!("Giá trị s: {}", s);  // ✅ vẫn dùng được
}
```

### 🧠 Tưởng tượng:

* `main` vẫn **giữ quyền sở hữu `s`**.
* Hàm `print_length` chỉ **mượn** `s` tạm thời, không sở hữu.

### 📊 Sơ đồ:

```
main ----> String("Xin chào")
  |
  |---&s---> print_length()  // Mượn, không chiếm quyền
```

---

## 🔁 2. **Mutable Borrowing – Mượn có thể thay đổi**

### ✔ Cho phép hàm thay đổi dữ liệu thông qua tham chiếu có thể thay đổi (`&mut`)

```rust
fn add_world(s: &mut String) {
    s.push_str(" thế giới!");
}

fn main() {
    let mut s = String::from("Xin chào");
    add_world(&mut s);  // mượn một cách mutable
    println!("{}", s);  // ✅ in: Xin chào thế giới!
}
```

### 🧠 Quy tắc quan trọng:

* **Chỉ được một mutable reference tại một thời điểm.**
* Không được vừa có immutable & mutable reference đồng thời.

### 📊 Sơ đồ:

```
main --(mutable borrow)---> add_world()
       (&mut s)                |
                                --> push_str vào String
```

---

## ❌ Ví dụ sai – Vi phạm quy tắc borrow:

```rust
fn main() {
    let mut s = String::from("Rust");

    let r1 = &s;
    let r2 = &mut s; // ❌ lỗi: không được borrow mutable khi đang có immutable borrow

    println!("{}, {}", r1, r2);
}
```

---

## 🧱 3. **Ownership với Struct**

```rust
struct User {
    name: String,
    age: u32,
}

fn print_name(user: &User) {
    println!("Tên: {}", user.name);
}

fn main() {
    let user1 = User {
        name: String::from("An"),
        age: 25,
    };

    print_name(&user1); // mượn struct, không chiếm ownership
    println!("Tuổi: {}", user1.age); // ✅ vẫn dùng được
}
```

### ✅ Nếu bạn chuyển ownership:

```rust
fn takes_user(user: User) {
    println!("Chào {}!", user.name);
}

fn main() {
    let user = User {
        name: String::from("Mai"),
        age: 20,
    };

    takes_user(user);
    // println!("{}", user.age); // ❌ Lỗi: user đã bị move
}
```

---

## 🧩 Tổng kết sơ đồ quy tắc mượn:

```
Quyền sở hữu (Ownership)
      |
      |---&T-------> Mượn bất biến (nhiều cái được)
      |
      |---&mut T---> Mượn biến đổi (chỉ 1 tại một thời điểm)
```

---

Nếu bạn muốn, mình có thể tạo sơ đồ hình ảnh hoặc demo tương tác (với `canvas` hoặc chia nhỏ phần học) để bạn dễ tiếp cận hơn. Bạn muốn tiếp tục với chủ đề nào tiếp theo?

