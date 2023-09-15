///
/// This example code tests 3 different types related to strings: String, &String, &str
///    - String: Owned string, or simply string.
///              Semantically equal to C++/Java string with buffer
///    - &String: Borrowed string
///    - &str: Borrowed string slice (or simple string slice).
///            &str or &mut str is the only way to interact with string slice `str`
///
#[test]
fn string_and_string_slice() {

    let mut hello: String = std::string::String::from("Hello");
    let name1 = String::from("Jongho");
    let whitespace: &str = " ";
    let name2 = String::from("Kim");
    let exclamation_mask: &str = "!";

    hello.push_str(", ");                // Literal is string slice `&str`
    hello.push_str(&name1);              // Borrow `String`
    hello.push_str(whitespace);          // Use borrowed string slice `&str`
    hello.push_str(name2.as_str());      // `as_str()` returns value of string slice `&str` type
    hello.push_str(&exclamation_mask);   // Borrowing borrowed string slice &(&str) -> &str

    println!("{}", hello);
}
