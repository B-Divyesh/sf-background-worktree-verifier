# Worktree Verifier

Run fast, declared smoke checks in separate Git worktrees. Worktree Verifier is
for developers who need to know which branch is fresh without switching
worktrees or waiting for CI.

It is a free, local Rust CLI. It watches configured directories, runs checks
serially, and serves a compact board on `127.0.0.1` with each worktree's state,
commit, and changed-file count. It has no account, telemetry, or hosted service.

## Install

Until published to crates.io, build it from a clone:

```sh
cargo install --path .
worktree-verifier --help
```

## Use it

Start with a commented config:

```sh
worktree-verifier init
```

Edit `.worktree-verifier.toml` to list each worktree and its fast, opt-in
checks:

```toml
[server]
address = "127.0.0.1:4318"
poll_seconds = 3

[[worktrees]]
name = "checkout-ui"
path = "../checkout-ui"
checks = ["npm test -- --runInBand"]
```

Run once in scripts or CI:

```sh
worktree-verifier run --once --json
```

Watch continuously and open the printed localhost URL:

```sh
worktree-verifier run
```

Checks run one at a time. This reduces accidental writes to shared test caches.
Worktree Verifier never discovers or runs commands automatically; only commands
in your config run. Review them before use.

## Try the isolated sample

```sh
cargo run -- demo
```

The demo creates `checkout-ui`, `checkout-api`, and `checkout-docs` under a
temporary directory, runs one declared smoke check in each, prints the location,
and removes the directory. Use `cargo run -- demo --keep` to inspect it or
`cargo run -- demo --serve` to open its localhost board.

The hosted documentation recording is available at `/demo` after running the
site locally. See [.factory/demo.md](.factory/demo.md) for its sandbox details.

## Develop, test, and build

Requirements: Rust stable and Node 20+.

```sh
npm install
npm test
npm run build:site  # writes static site to dist/site
cargo package       # validates the publishable crate; does not publish
```

To develop the documentation site:

```sh
npm run dev
```

`npm run build` is an alias for `npm run build:site`. The deploy artifact is
`dist/site`, with `index.html` at its root.

## Privacy and boundaries

The CLI binds its board to loopback by default. Its own code makes no network
requests. A command you configure may make network requests; that is under your
control. Worktree Verifier does not replace CI, write tests, or synchronize
branches.

The documentation site is static and has no analytics. Its `/privacy` and
`/terms` routes are included in the built site.

## License

[MIT](LICENSE)
