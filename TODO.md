## Redesign

The aim of the redesign is to improve the flexibility and expressiveness of `oneshot`.

### Configuration

Default or per-tool settings:

- [ ] **default_envs**: Global environment variables to be injected in every
      container run
- [ ] **default_mounts**: Global mounts applied for all tools unless
      overridden (default is cwd and temp).
- [ ] **working_dir**: Directory inside a container where commands are run
      (default: `/workspace`)
- [ ] **shell**: Default shell (bash, sh) for running multi-command steps
- [ ] **dotfiles**: Dotfiles to use
- [ ] **timeout**: Max duration a step or pipeline can run before aborting

Cli behaviour

- [ ] verbose
- [ ] dry_run
- [ ] profile: Switch config profiles or overrides

### Tool config

The configuration file contains the registry of tools

```sh
[tool.brew]
container = "ghcr.io/homebrew/brew"
envs = ["RUST_LOG=debug", "OTHER_ENV=val"]
mounts = ["cwd:rw", "ssh-agent:ro"]
install = "brew install {packages}"
run = "{command}"
```

So the user can run things really easily:

```sh
oneshot --tool brew --install ffmpeg --run "ffmpeg input.m4a output.m4a"
```

- [ ] Container image/tag version pinning

### Pipeline config

Later, pipelines can be added to pass the output of one container into another.

```toml
[pipeline]
name = "media_processing"

[[pipeline.steps]]
id = "install_deps"
tool = "brew"
install = "ffmpeg"

[[pipeline.steps]]
id = "convert_audio"
tool = "brew"
run = "ffmpeg input.m4a output.m4a"
depends_on = ["install_deps"]

[[pipeline.steps]]
id = "analyze_audio"
tool = "python"
run = "python analyze_audio.py output.m4a"
depends_on = ["convert_audio"]
```

```sh
oneshot pipeline run --name media_processing
```

Config

- [ ] Retries
- [ ] On failure
- [ ] Parallel steps
- [ ] Logging
- [ ] Hooks
