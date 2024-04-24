# A simple app demo how to use redis in rust

## Prepare
```bash
sudo apt update && sudo apt install redis -y

sudo systemctl status redis
/usr/bin/redis-server --version
```

## How to build
```bash
cargo build
```

## How to run
[1st terminal]
```bash
cargo build
cargo run subscriber test_channel
```

[2nd terminal]
```bash
cargo run publisher test_channel hello
cargo run publisher test_channel 'how are you?'
cargo run publisher test_channel bye
```

## Reference:
1. https://docs.rs/redis/latest/redis/