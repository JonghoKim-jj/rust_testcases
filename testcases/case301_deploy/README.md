# MacOS

``` sh
brew install colima
```

``` sh
# This will fail if docker daemon is not started
docker ps

# This will work
colima start
docker ps
```

# Reference
- [Can Cargo download and build dependencies without also building the application?](https://stackoverflow.com/questions/42130132/can-cargo-download-and-build-dependencies-without-also-building-the-application)
- [Docker로 Rust 애플리케이션 배포하기](https://marshallku.com/dev/deploy-rust-with-docker)
- [cargo-build-dependencies](https://crates.io/crates/cargo-build-dependencies)
- [cargo-build-deps](https://crates.io/crates/cargo-build-deps)
