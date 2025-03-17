```bash
cargo clean
```

```bash
cargo build
```

```bash
cargo build --release
```

# Run tests
```bash
cargo test
```

```bash
curl -X POST http://localhost:3000/cache -H "Content-Type: application/json" -d '{"key": "username", "value": "rustacean"}'
```

```bash
curl -X GET http://localhost:3000/cache/username
```

```
http://127.0.0.1:3000/swagger
```

# Docker

```sh
docker build --tag 'sqlite-axum-cache:0.0.1' .
```

# Docker compose

```shell
docker-compose up -d
```

```shell
docker-compose down -v
```