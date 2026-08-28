# Worktree Verifier sample

Run `worktree-verifier demo` to create these three representative temporary
worktrees at runtime: `checkout-ui`, `checkout-api`, and `checkout-docs`.
Each has one small declared check. The command prints their temporary location
and removes it unless `--keep` is supplied.

The files are not copied into a real repository and the demo command does not
read the current directory.
