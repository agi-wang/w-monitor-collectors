# W Monitor Collectors

Shared read-only collection library. Preserve existing JSON collector fields and units. Use minimal changes and no Private SDK, transport, identity, command dispatcher, control listener, remote file/terminal handling, privilege escalation, installation or update code/features. Bounded local diagnostic queries and sampling caches are collection only.

Do not write or modify unit/integration tests. Preserve consumers' existing checks; use GitHub Actions for formatting, clippy and locked compile/package acceptance. Keep local checkouts limited to source editing and metadata checks. Do not deploy either consumer or bump its release as part of library maintenance. Both consumers pin reviewed immutable revisions and release independently.

Canonical development source is https://github.com/agi-wang/w-monitor-collectors. Submit source changes and contributions directly through GitHub pull requests; GitHub Actions owns build, acceptance and release work. Never publish credentials, runtime configs, collected host data or private application source.
