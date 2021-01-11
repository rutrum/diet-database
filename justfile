test:
    cargo test --all

watch-test:
    watchexec -- just test

build:
    cargo build --all

web:
    (cd web; wasm-pack build --target web --out-name package --dev)

watch-web:
    watchexec -w web/src -- just web

serve:
    (cd web; microserver)

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

css:
    (cd web; grass scss/index.scss > index.css)

watch-css:
    watchexec -w web/scss -- just css

dev:
    tmux kill-session -t diet || true
    tmux new-session -d
    tmux rename 'diet'
    tmux renamew 'css-serve'
    tmux send 'just watch-css' ENTER
    tmux new-window -n 'serve'
    tmux send 'just serve' ENTER
    tmux new-window -n 'api'
    tmux send 'just watch-api' ENTER
    tmux new-window -n 'web'
    tmux send 'just watch-web' ENTER
    tmux attach -t diet

schema:
    diesel print-schema
