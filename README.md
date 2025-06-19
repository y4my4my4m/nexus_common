![Screenshot_20250618_160320](https://github.com/user-attachments/assets/ebb945fe-7aa7-4eaa-988f-5a8595140314)

https://github.com/y4my4my4m/cyberpunk_bbs_server

https://github.com/y4my4my4m/cyberpunk_bbs_client

---

## Build

```bash
cargo build -p client --release # for arch/manjaro (glibc)
cargo build --release --target x86_64-unknown-linux-musl # for void linux (musl)
```

```bash
cp target/release/client ./dist/client-linux-arch
cp target/x86_64-unknown-linux-musl/release/client ./dist/client-linux-void
```

## Dev

```bash
cargo run --release -p client
cargo run --release -p server
```
