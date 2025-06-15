///
/// #[cfg(test)] on module tests tells Rust
/// to compile and run when you run `cargo test`,
/// not when you run `cargo build`
///
#[cfg(test)]
mod tests {

    #[test]
    fn test_hello() {
        println!("Hello, unit test!");
    }

    #[test]
    #[should_panic]
    fn test_intentional_panic() {
        panic!("INTENTIONAL PANIC!");
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        println!("This test will be ignored unless explicitly requested")
    }

    #[test]
    fn test_assertion() {
        assert_eq!((2+3), 5);
    }
} // mod tests
