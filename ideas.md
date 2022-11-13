# Steps

- build a logger for error handling
- implement storage
  - could use https://github.com/dragonflydb/dragonfly for storage
  - cache boards and lists
- build UI
- implement keybinding
- time management https://lib.rs/crates/time
- update ui on ticks
- make adaptable ui to sizes and info available
- build proper project setup with roadmap etc
- use tokio for async loading of data?
- editing descriptions https://lib.rs/crates/tempfile

# Refactoring

check all implementations for
allocations
and
call by reference / borrowing

> bacon + clippy

# readme:

i know I could ve used some more libraries, but I wanted to learn rust!

# Questions

What is the builder pattern that bottom uses for app.rs?
