FROM rust:slim-bullseye

# Build dependencies only.
RUN cargo init foo
COPY Cargo.toml foo/
RUN cargo build --release; \
    rm -rf foo

# Install `foo`.
COPY . .
RUN echo "// force Cargo cache invalidation" >> foo/src/main.rs; \
    cargo install --path foo

CMD ["foo"]
