# 0.3.0 (2024-03-25)

In this release `TTL` command got added. Additionally, updated other commands to reflect the change.

- `TTL` is the new command that enables checking for time to live e.g. `TTL <key>`
- `GET`, `DELETE` and `TTL` commands remove entries from both HashMaps (expiries and data store) when called with an expired key.

# 0.2.0 (2024-03-15)

Added `SET` command feature.

### Features

- `SET` command now can set string that contain spaces e.g. `SET greeting "hello world"`

# 0.1.0 (2024-02-24)

Create CHANGELOG to track changes and enhancements.

### Features

- `PING`, `GET`, `SET`, `DELETE` commands supported
- `PING` takes in a value that then gets returned i.e. `PING hello`
