## Redesign

Going to redesign `oneshot` with a focus on caching instead.

Oneshot will;
1. Generate a dockerfile, based on what the user passes in
2. Run that dockefile with the script

Store dockerfiles in;
~/.oneshot/dockerfiles/<hash>/

Maybe: periodically prune unused containerfiles

## TODO

- [ ] Create Dockerfile generation script
- [ ] Create part that runs Dockerfile
- [ ] Config file to choose Docker or Podman

## Ideas

- `--keep-alive`: Do not close container on exit
- `toml` Allows a oneshot to be configured with a toml file
- Piping oneshots into each other `oneshot -s "generate data" | oneshot -s "process data"`
- Use Oneshot as a shim package manager
- Change focus of Oneshot to just package manager for containerised scripts

## Future direction

- Oneshot focused on managing packages/libraries (adding binaries/script to PATH
or exposing commands with `oneshot run`).
- 
