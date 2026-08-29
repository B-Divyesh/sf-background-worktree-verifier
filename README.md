# Worktree Verifier

Run fast, declared smoke checks in separate Git worktrees. It is for developers
who need to know which branch is fresh without switching worktrees.

It watches configured Git worktrees and serves a compact board on `127.0.0.1`
with each worktree's state, commit, and changed-file count.

## Install

Until published to crates.io, build it from a clone:

```sh
git clone https://github.com/B-Divyesh/sf-background-worktree-verifier.git
cd sf-background-worktree-verifier
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

Worktree Verifier never discovers or runs commands automatically; only commands
in your config run. Review them before use.

## Fresh results and command boundaries

Before a smoke check starts, the watcher snapshots the worktree's commit and
working state. If either changes while the command runs, it marks that attempt
stale and checks the new snapshot before it can report a pass. The local board
shows the current snapshot and `last_pass_commit`, so a later failure never
erases the last known passing commit.

The watcher reruns only the configured worktree whose Git state changed. It
does not discover commands or repositories for you.

Configured commands run with the permissions of the account that starts the
CLI. There is no hidden command sandbox. For an explicit Linux filesystem and
network boundary, wrap a check with [Bubblewrap](https://github.com/containers/bubblewrap)
when it is available on your machine:

```toml
checks = ["bwrap --die-with-parent --unshare-net --ro-bind /usr /usr --ro-bind /lib /lib --ro-bind /lib64 /lib64 --dev /dev --proc /proc --bind \"$PWD\" /work --chdir /work sh -lc 'cargo test'"]
```

This example makes the worktree writable at `/work`, does not mount your home
directory, and blocks network access. Adapt the read-only runtime mounts for
your operating system and toolchain.

## Try the isolated sample

```sh
cargo run -- demo
```

The demo creates `checkout-ui`, `checkout-api`, and `checkout-docs` as actual
Git worktrees under a temporary directory. It commits a sample file and runs one
declared smoke check in each, prints the location, and removes the directory.
Use `cargo run -- demo --keep` to inspect it or `cargo run -- demo --serve` to
open its localhost board.

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

The CLI binds its board to loopback by default. The status page confirms a
loopback-only listener and warns when your configured address may accept network
connections. A command you configure may make network requests; review each
command before adding it.

The documentation site is static and sends no analytics or tracking requests.
Its `/privacy` and `/terms` routes are included in the built site.

## Deploy

Build the static documentation artifact with `npm ci && npm run build`. Deploy
the generated `dist/site` directory with the included
`staticwebapp.config.json`; deployment infrastructure is managed by the
factory.

## License

[MIT](LICENSE)
