test:
    cargo test --all

watch-test:
    watchexec -- just test

build:
    cargo build --all

#web:
#    (cd client; wasm-pack build --target web --out-name package --dev)

#watch-web:
#    watchexec -w client/src -- just web

#serve: web
#    (cd client; microserver)

tree:
    tree -I "pkg|target" --dirsfirst

api:
    cargo run -p api --bin api

watch-api:
    watchexec -w api/src -- just api

fmt:
    cargo fmt

watch-check:
    watchexec -- cargo check --workspace

#css:
#    (cd client; grass scss/index.scss > index.css)

#watch-css:
#    watchexec -w client/scss -- just css
