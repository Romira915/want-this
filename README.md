# Want This

## Setup cargo

```bash
cargo install --locked trunk
cargo install --locked wasm-bindgen-cli
cargo install cargo-watch
rustup target add wasm32-unknown-unknown
```

## Database migration

```bash
mysqldef -uwantthis -h127.0.0.1 -pwant_pass wantthis < sql/migration.sql
```

## Starting Debug

```bash
yarn dev
```
