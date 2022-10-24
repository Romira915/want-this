# Want This

## Setup cargo subcommand

```bash
cargo install --locked trunk
cargo install --locked wasm-bindgen-cli
cargo install cargo-watch
```

## Database migration

```bash
mysqldef -uwantthis -h127.0.0.1 -pwant_pass wantthis < sql/migration.sql
```

## Starting Debug

```bash
yarn dev
```
