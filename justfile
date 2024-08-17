@default: 
  just --list

@watch:
  cargo watch -x "run --release"

@build:
  cargo build --release
  cp ./target/release/www-backend ./www-bin

@update: build
  mv ./www-bin ../www-frontend/
