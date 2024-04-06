# Tokio mini-redis
```bash
cargo install mini-redis
sudo systemctl status redis-server
sudo systemctl stop redis-server
mini-redis-server
mini-redis-cli get foo
cargo new my-redis
cd my-redis
cargo add tokio --features full
cargo add mini-redis
cargo add bytes
```
## Hello Tokio
- cargo run --example hello-redis
## Spawning
## Shared State
## [Channels](my-redis/src/main.rs)
- cargo run --bin my-redis
## [I/O](my-redis/src/bin/echo-server-copy.rs)
- cargo run --bin echo-server-copy
## Framing
## Async in Depth