# Horace

An attempt to replicate the functionality of other Discord bots in a way that better
suits the needs of the RPS Discord.
Made using the [Serenity](https://crates.io/crates/serenity) library.

## TODO

- [ ] Info channel maintenance (welcome, happenings)
- [x] Statistics
- [x] Roles
    - [x] Adding and removing roles
    - [x] Listing roles
    - [x] Only certain roles joinable
- [x] Reminders
- [ ] Custom commands for simple messages
- [ ] Admin commands (kick, ban, purge, etc.)
- [ ] Silent place / mute
- [ ] Configuration and settings

## How to run

First, set `DISCORD_TOKEN` and `POSTGRES_CONNSTRING` in your environment.
If the latter is ommitted, horace will run while complain constantly. Then:

```sh
`cargo run --release`.
```
