If you want to build, then run:
```sh
cargo build
```


If you want to build and run, then run:
```sh
cargo run
```

If you want to run unit tests, then run:
```sh
cargo test
```

If you want to run unit tests and see messages, you may use double hyphens without option name or value:
```sh
cargo test -- --nocapture
```

or

```sh
cargo watch "test -- --nocapture"
```

Note that running unit tests will not execute the `main()` function


If you want to run ignored unit tests, then run:
```sh
cargo test -- --ignored
```

If you want to all unit tests including ignored tests, then run:
```sh
cargo test -- --include-ignored
```
