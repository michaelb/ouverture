# the dev folder

contains the necessary stuff (config, local db)

high-level server/ui behavior run cmds:

- with config `external_server=true` and `background=true`
`./target/release/ouverture-server -c dev/config.toml --log-level trace && ./target/release/ouverture -c dev/config.toml`

- with config `external_server=true` and `background=false`
`./target/release/ouverture-server -c dev/config.toml --log-level trace & ./target/release/ouverture -c dev/config.toml`

- with config `external_server=false`
`./target/release/ouverture -c dev/config.toml`
