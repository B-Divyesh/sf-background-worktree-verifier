# Worktree Verifier

Run declared checks in separate Git worktrees. It is for developers who need
current branch results without switching worktrees.

The local board shows each worktree's state, commit, and changed-file count.

## Install

Build it from a clone:

```sh
git clone https://github.com/B-Divyesh/sf-background-worktree-verifier.git
cd sf-background-worktree-verifier
cargo install --path .
worktree-verifier --help
```

## Configure and run Worktree Verifier

Create a commented config:

```sh
worktree-verifier init
```

Edit `.worktree-verifier.toml` to list each worktree and its configured checks:

```toml
command_timeout_seconds = 60

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

The board starts before the first check and shows `RUNNING` while commands run.
Each command stops after `command_timeout_seconds`. A timeout appears as
`ERROR`; changing that worktree makes the watcher try its checks again.

Worktree Verifier never discovers or runs commands automatically; only commands
in your config run. Review them before use.

## Fresh results and command boundaries

Before a check starts, the watcher snapshots the worktree's commit and working
state. If either changes during a check, the watcher marks that result stale.
It checks the new snapshot before reporting a pass. The local board shows the
current snapshot and `last_pass_commit`. A later failure never erases the last
known passing commit.

The watcher reruns only the configured worktree whose Git state changed. It
does not discover commands or repositories for you.

The CLI adds no isolation layer. Configured commands inherit its user identity,
environment, and filesystem access. Use an operating-system sandbox when a
command needs a stricter boundary.

## Try the isolated sample

```sh
cargo run -- demo
```

The demo creates `checkout-ui`, `checkout-api`, and `checkout-docs` as separate
Git worktrees under a temporary directory. It commits one sample file in each
worktree. It runs one declared check in each, prints the location, and removes
the directory. Use `cargo run -- demo --keep` to inspect the files. Use `cargo
run -- demo --serve` to open its board on this computer.

Open `/?demo=1` for the hosted recording after running the site locally. The
same recording is also available at `/demo`. See
[.factory/demo.md](.factory/demo.md) for its sandbox details.

## Develop, test, and build

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

By default, only this computer can open the board at `127.0.0.1`. The status
page warns when another device may reach the configured address. A configured
command keeps its network access. Review each command before adding it.

The documentation site is static and sends no analytics or tracking requests.
Its `/privacy` and `/terms` routes are included in the built site.

## Deploy

Build the static documentation artifact with `npm ci && npm run build`. Deploy
the generated `dist/site` directory with the included
`staticwebapp.config.json`; deployment infrastructure is managed by the
factory.

## License

[MIT](LICENSE)
