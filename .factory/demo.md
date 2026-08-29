# Demo sandbox

Open `/?demo=1` for the one-click documentation demo. `/demo` opens the same
isolated recording. Use **Replay sample** to advance its terminal frames, or
**Reset demo** to restore the complete initial output, focus, and scroll.

The demo banner stays visible until **Start for real** leaves demo mode. The
recording writes no localStorage, sessionStorage, IndexedDB, or service-worker
data. It does not read real browser data.

Run `cargo run -- demo` from a clean checkout for the real CLI sample.

The command initializes an isolated seed Git repository and creates three Git
worktrees under the system temporary directory. It commits one sample file in
each and runs one declared `test -f` check in each. It prints the location,
then removes the directory. `cargo run -- demo --keep` leaves it for
inspection. `cargo run -- demo --serve` starts the same sample watcher and
board at `http://127.0.0.1:4319`.

The CLI sample never touches the caller's repository. The browser demo keeps
no storage namespace because it persists no state.
