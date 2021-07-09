# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit

### Example ~/.orbiter.config.yml

```yaml

- id: ff-dev
  resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
  extract: "tar xvf *.tar.*"
  exec: "**/firefox/firefox"
  launcher:
    name: firefox
    exec: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
    icon: firefox
    menu_name: Firefox

- id: tmux
  resource:
    repo: tmux/tmux
    ver: "3.2a"
  install: "./autogen.sh; ./configure; make;"
  exec: "**/tmux"

- id: neovim
  resource:
    repo: neovim/neovim
    is_release: true
  extract: "tar xvf *.tar.*"
  exec: "**/bin/nvim"

- id: exa
  resource:
    repo: ogham/exa
    is_release: true
  extract: "unzip *.zip"
  exec: "**/exa"

- id: zellij
  resource:
    repo: zellij-org/zellij
    is_release: true
  extract: "tar xvf *.tar.*"
  exec: "**/zellij"

- id: direnv
  resource:
    repo: direnv/direnv
    is_release: true
  install: "mv direnv* direnv; chmod +x ./direnv; ./direnv hook zsh > zhook.zsh"
  src: zhook.zsh
  load: export DIRENV_LOG_FORMAT=""
  exec: "**/direnv"

```

### Order of Execution

(If not already exist: `clone`/`update` (if not exist) -> `extract` -> `install`)

`(plugin script loading)` -> `src` -> `multisrc` -> `load`


