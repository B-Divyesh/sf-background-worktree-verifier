# Landing copy audit

Audited route: `/` in `site/src/main.js`. Headings and labels are included so
the reading order can be checked. No sentence exceeds 22 words. No banned word
appears.

| Copy | Words | Result |
| --- | ---: | --- |
| Check changed worktrees in the background | 6 | pass |
| For developers with separate branches who need fresh smoke results without switching worktrees. | 11 | pass |
| See three Git worktree checks pass. | 6 | pass |
| Sample uses isolated Git worktrees. | 5 | pass |
| Commands are opt-in. | 3 | pass |
| Board defaults to localhost. | 4 | pass |
| Separate changes. | 2 | pass |
| One fresh status board. | 4 | pass |
| See every worktree at a glance | 7 | pass |
| Commit hashes and changed-file counts make freshness explicit. | 7 | pass |
| Run smoke checks where the changes live | 8 | pass |
| Give each Git path and each fast command in one file. | 12 | pass |
| The CLI checks each worktree after its files change. | 10 | pass |
| Open localhost to see passes, failures, commits, and changed files. | 10 | pass |
| Your checks stay local and intentional | 6 | pass |
| The CLI runs only commands you put in its config. | 10 | pass |
| The status board binds to localhost by default. | 8 | pass |

## Terminology

| Concept | One word used |
| --- | --- |
| Isolated Git directory | worktree |
| Fast user-declared command | smoke check |
| Local web summary | status board |
| A configured command list | config |
| Files that differ from the commit | changed files |
