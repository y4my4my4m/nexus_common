![Screenshot_20250620_025053](https://github.com/user-attachments/assets/040cdaec-b141-4280-9b57-c8558770bf1d)

https://github.com/y4my4my4m/cyberpunk_bbs_server

https://github.com/y4my4my4m/cyberpunk_bbs_client

---

## Build

```bash
cargo build -p client --release # for arch/manjaro (glibc)
cargo build -p client --target x86_64-unknown-linux-musl --no-default-features --release # for void linux (musl)
```

```bash
cp target/release/client ./dist/nexus-client-linux-arch
cp target/x86_64-unknown-linux-musl/release/client ./dist/nexus-client-linux-void
```

## Dev

```bash
cargo run --release -p client
cargo run --release -p server
```

## Run with remote server

```bash
# client
./nexus-client-linux-arch 199.192.20.57:8080

# server
cargo run --release -p server 0.0.0.0:8080
```
