default: watch

watch:
  @cargo watch -x "run --release"

build:
  @cargo build --release
  @cp ./target/release/portfolio-backend .

update:
  @just build
  @mv ./portfolio-backend ../portfolio-frontend/portfolio-bin
