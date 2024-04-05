# mini-redis
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
...
cargo run --example hello-redis
```