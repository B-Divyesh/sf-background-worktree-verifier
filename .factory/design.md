# Worktree Verifier design system

## Direction

**Dithered / halftone print system.** Worktree Verifier feels like a folded
overnight test bulletin pinned beside a developer's terminal: ink, paper,
registration marks, and clear status stamps. The texture explains the job:
many separate change streams become one readable board. It avoids glossy CI
dashboard language and generic SaaS panels.

## Palette

| Token | Value | Use |
| --- | --- | --- |
| paper | `#f5eedb` | warm page background |
| ink | `#17202b` | headings and body text |
| muted-ink | `#48515a` | supporting copy |
| navy | `#183a59` | header, terminal, links |
| vermilion | `#b84531` | primary action and failures |
| moss | `#25674e` | passing status |
| ochre | `#a36313` | running / stale status |
| line | `#b8ad94` | rules and registration marks |

The site is deliberately single-mode: warm stock is part of the print thesis.
All combinations use dark ink on paper or white paper on navy and meet the
normal-text contrast target.

## Type and rhythm

`Georgia` is the editorial display face; `ui-monospace` is the operational
face used for commands, commits, and status. Both are local system font stacks,
so there is no font download. Spacing follows an 8px scale: 8, 16, 24, 32, 48,
72, 96. Body text is 18px / 1.55 and content stays below 68 characters.

## Interaction and motion

Links are underlined; controls are square, ink-stamped shapes with an
offset-shadow press response. Status dots use a one-time 180ms entrance when a
board appears. Under `prefers-reduced-motion`, transitions and animation are
removed. Keyboard focus is a 3px vermilion outline.

## Asset plan and provenance

`site/public/halftone-worktrees.webp` is an original generated editorial
illustration: three small worktree folders feed a central check board, printed
with a coarse navy, vermilion, and moss halftone. It contains no readable text.
Generated for this product with `/opt/fleet/lib/gen-image.sh` using the
factory-image deployment; optimized to WebP under 300 KB. The terminal
recording is a hand-authored SVG, not a third-party asset. Small rules, dots,
and registration marks are CSS/SVG made in this repository.
