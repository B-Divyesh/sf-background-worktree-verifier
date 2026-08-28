# Demo sandbox

Run `cargo run -- demo` from a clean checkout.

The command creates three sample worktree folders in the system temporary
directory, runs a declared `test -f` smoke check in each, prints the location,
then removes the folder. `cargo run -- demo --keep` leaves it for inspection.
`cargo run -- demo --serve` starts the same sample watcher and board at
`http://127.0.0.1:4319`.

The documentation demo is `/demo`. It is a self-hosted terminal recording of
that command. It writes no browser storage and no real data is read or saved.
