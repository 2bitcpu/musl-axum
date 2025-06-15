# musl-axum

```
cargo init --name web-api

cargo add tokio --features macros,rt-multi-thread,signal --no-default-features
cargo add serde --features derive --no-default-features
cargo add serde_json --features std --no-default-features
cargo add chrono --features serde,now --no-default-features
cargo add async-trait --no-default-features
cargo add sqlx --features runtime-tokio-rustls,chrono,derive,sqlite --no-default-features
cargo add axum --features macros
cargo add axum-extra --features typed-header --no-default-features
cargo add tower --features timeout --no-default-features
cargo add tower-http --features fs,cors --no-default-features
cargo add derive-new --no-default-features
cargo add libsqlite3-sys@^0.30.1 --optional --no-default-features

cat << EOS >> Cargo.toml

simple-jwt = { git = "https://github.com/2bitcpu/simple-jwt" }
async-argon2 = { git = "https://github.com/2bitcpu/async-argon2" }

[profile.release]
opt-level = "z"
debug = false
lto = true
strip = true
codegen-units = 1
panic = "abort"

# cargo +nightly-2025-02-20 build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target aarch64-unknown-linux-gnu --release
# upx --best --lzma ./target/aarch64-unknown-linux-gnu/release/web-api
EOS

cat << EOS >> migrate.sql
CREATE TABLE IF NOT EXISTS content (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    pablish_at DATETIME,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);
EOS
```

### Build commsnd
```
docker run --rm -it --mount type=bind,source="$(pwd)",target=/project -w /project messense/rust-musl-cross:aarch64-musl cargo build --release
```

### Execute command
```
docker run --rm -it --mount type=bind,source="$(pwd)",target=/project -w /project -p 3000:3000 gcr.io/distroless/static-debian12 /project/target/aarch64-unknown-linux-musl/release/web-api 
```

### Test Command
```
curl -i -X POST -H 'Content-Type: application/json' -d '{"id":0,"publishAt":"2025-06-10T01:02:03Z","draft":true,"title":"a","body":"b"}' http://localhost:3000/service/content/create
```
