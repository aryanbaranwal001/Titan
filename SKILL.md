# Working Style

1. Keep the code minimal, the focus is the happy path working correctly, not production hardening.
2. Write learning comments using the language's native single-line syntax with ## immediately after the comment marker, so that these comments can be easily grep/strip later. For example `Rust / Go / TS / JS → // ##`
3. Learning comments should be done in a way so that developer understands it in one read.
4. Add dev logs at each steps so that the flow is clearly visible at runtime. Use `eprintln!"[dev] ... "` or `println!"[dev] ... "` for dev-only traces (easy to grep/strip later)
5. When there is a `// ## Q` comment in the code, answer it just after the question, with a `// ## A` comment directly below the question.
6. Keep the answer concise and grounded in what the code is actually doing. No need to re-explain anything already answered this way in a future conversation. Keep the question intact. Answer it after the question.
7. When the user asks for a commit message, run `git diff --cached --stat` and suggest one concise message for the staged changes using conventional commits format: `type: short description` — e.g. `feat: wire mock block stream into clickhouse via mapping`. Pick the type from: feat, fix, refactor, chore, docs.
