# Welcome to the project!

Contributors are very welcome and I hope you'll have a great time here!

Don't forget to check out the [Code of conduct](CODE_OF_CONDUCT.md)

Also, please note that any contribution shall be licensend as MIT, and that the contributor will appear as such on the project front page and git history.


## Setup your development environment

- You'll need first: An editor, git and the latest Rust stable toolchain
- Install the dependencies mentionned in the project's [README](README.md)

- [Fork the project](https://github.com/michaelb/ouverture/fork)
- Clone it to your local dev environment
- [optionnal] setup the rustfmt git hook: `git config core.hooksPath .githooks`

Then, make some modification, push them to your fork and submit a pull request from there


## Before submitting

- Format your code `cargo fmt --all`
- Ensure all the unit tests pass: this will be checked by the CI `cargo test --release`



## Logging behavior

By ascending order of importance:

trace, debug, info, warn, error

We'll use log level as follows:

- trace: spammy information from ouverture
- debug: Basic information, useful for debug 
- info: Notable info about Ouverture, silencing 'info' log level from dependencies
- warn: Recoverable failures
- err: irrecoverable failures

