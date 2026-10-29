# W Monitor Collectors

Shared read-only collection library. Preserve existing JSON collector fields and units. Use minimal changes and no Private SDK, transport, identity, command dispatcher, control listener, remote file/terminal handling, privilege escalation, installation or update code/features. Bounded local diagnostic queries and sampling caches are collection only.

Do not write or modify unit/integration tests. Preserve consumers' existing checks; use formatting, clippy, locked compile/package checks and bounded manual collection for acceptance. Do not deploy either consumer or bump its release as part of library maintenance. Both consumers pin reviewed immutable revisions and release independently.

Read and follow workspace instructions. Canonical development source is on Gitea; GitHub publication must use `w-cli github export` and inspection. Never publish credentials, runtime configs, collected host data or private application source. Record durable findings in Radar project `w-monitor-collectors`.
