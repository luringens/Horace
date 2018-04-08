# Horace

An attempt to replicate the functionality of other Discord bots in a way that better suits the needs of the RPS Discord. Made using the [Serenity](https://crates.io/crates/serenity) library.

## TODO

- [ ] Welcome page
- [ ] Happenings page
- [x] Statistics
- [x] Roles
    - [x] Adding and removing roles
    - [x] Listing roles
    - [x] Only certain roles joinable
- [x] Reminders
- [ ] Custom commands
    - [ ] Trigger on keywords
    - [ ] Restrict to channels
    - [ ] Custom message
- [ ] Kicking, banning
- [ ] Silent place
- [ ] Web configuration?
- [ ] Connection pooling to speed up database access
- [ ] Proper logging

## How to run

First, set `DISCORD_TOKEN` and `POSTGRES_CONNSTRING` in your environment. If the latter is ommitted,
horace will run while complain constantly. Then:

```sh
`cargo run --release`.
```
