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

## Localhost TLS

```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes \
  -subj "/CN=localhost" \
  -addext "subjectAltName=DNS:localhost,IP:127.0.0.1" \
  -addext "basicConstraints=critical,CA:FALSE"
```

## Server TLS script

```bash
#!/bin/bash
set -e

# 1. Install certbot if not present
if ! command -v certbot &> /dev/null; then
    echo "Installing certbot..."
    sudo apt-get update
    sudo apt-get install -y certbot
fi

# 2. Obtain a certificate (replace example.com with your domain)
sudo certbot certonly --standalone -d example.com

# 3. Copy/convert certs for your Rust server
sudo cp /etc/letsencrypt/live/example.com/fullchain.pem /home/youruser/gits/hobby/nexus/cert.pem
sudo cp /etc/letsencrypt/live/example.com/privkey.pem /home/youruser/gits/hobby/nexus/key.pem
sudo chown youruser:youruser /home/youruser/gits/hobby/nexus/cert.pem /home/youruser/gits/hobby/nexus/key.pem

# 4. Start your server (adjust path as needed)
cd /home/youruser/gits/hobby/nexus
cargo run --release --bin server -- 0.0.0.0:443
```