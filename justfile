@default: 
  just --list

@watch:
  cargo watch -x "run --release"

@build:
  cargo build --release
  cp ./target/release/portfolio-backend .

@update: build
  mv ./portfolio-backend ../portfolio-frontend/portfolio-bin
