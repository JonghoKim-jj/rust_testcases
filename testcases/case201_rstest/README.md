This test case describes how to use `rstest` crate to test.    
`rstest` is a fixture-based test framework for Rust.  

You can read [rstest documentation](https://docs.rs/rstest/latest/rstest/attr.rstest.html).  
It says the minimum Rust version for `rstest` is 1.67.1

First you should include `rstest` to `dev-dependencies` of `Cargo.toml`.

``` toml
[dev-dependencies]
rstest = "0.23.0"
```

You can use inside test module:

```rust
// Module defined here
// ...

// mod test: Unit tests for the module is defined here
#[cfg(test)]
mod test {
    // dev-dependencies here
    use rstest::rstest;

    // Test code here
    // ...
}
```

You can test with:
```sh
cargo t
```

You can test `mult` module only:
```sh
cargo t mult
```