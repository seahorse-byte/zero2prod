# github repo

```bash
cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres
```

```bash
./scripts/init_db.sh
```

```bash
cargo run
```

```bash
curl http://127.0.0.1:8000/health_check -v
```

```bash
curl -i -X POST  -d 'email=olsi@hotmail.com&name=Olsi Gjeci' \
http://127.0.0.1:8000/subscriptions
```

[https://github.com/LukeMathWalker/zero-to-production/](https://github.com/LukeMathWalker/zero-to-production/)

```bash
# Tests
TEST_LOG=true cargo test health_check_works | bunyan

TEST_LOG=true cargo test | bunyan
```
