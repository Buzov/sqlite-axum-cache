```bash
cargo clean
```

```bash
cargo build
```

```bash
cargo build --release
```

```bash
curl -X POST http://localhost:3000/cache -H "Content-Type: application/json" -d '{"key": "username", "value": "rustacean"}'
```

```bash
curl -X GET http://localhost:3000/cache/username
```