# Demo sandbox

Run `cargo run -- demo` from a clean checkout.

The command initializes an isolated seed Git repository, creates three actual
Git worktrees under the system temporary directory, commits one sample file in
each, runs a declared `test -f` smoke check in each, prints the location, then
removes the directory. `cargo run -- demo --keep` leaves it for inspection.
`cargo run -- demo --serve` starts the same sample watcher and board at
`http://127.0.0.1:4319`.

The documentation demo is `/demo`. It is a self-hosted terminal recording of
that command. It writes no browser storage and no real data is read or saved.
