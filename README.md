# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit

### Example ~/.orbiter.config.yml

```yaml

- id: diff-so-fancy
  resource:
    repo: so-fancy/diff-so-fancy
    is_release: true
  install: "git config --global core.pager \"diff-so-fancy | less --tabs=4 -RFX\"; git config --global interactive.diffFilter \"diff-so-fancy --patch\""
  exec: "**/diff-so-fancy"

- id: oh-my-tmux
  resource:
    repo: gpakosz/.tmux
  install: "ln -sf $PWD/.tmux.conf $HOME/.tmux.conf"

- id: tpm
  resource:
    repo: tmux-plugins/tpm
  install: "mkdir -p ~/.tmux/plugins; ln -sf $PWD/ $HOME/.tmux/plugins/tpm"

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

- id: vim-plug
  resource:
    repo: junegunn/vim-plug
  install: "mkdir -p ~/.local/share/nvim/site/autoload;  ln -sf \"$PWD/plug.vim\" ~/.local/share/nvim/site/autoload/plug.vim"

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

- id: gh
  resource:
    repo: cli/cli
    is_release: true
  extract: "tar xvf *.*gz"
  exec: "**/gh"

- id: exercism
  resource:
    repo: exercism/cli
    is_release: true
  extract: "tar xvf *.*gz"
  exec: "**/exercism"

- id: dprint
  resource:
    repo: dprint/dprint
    is_release: true
  extract: "tar xvf *.*gz"
  exec: "**/dprint"

- id: fzf
  resource:
    repo: junegunn/fzf-bin
    is_release: true
  extract: "tar xvf *.*gz"
  exec: "**/fzf"


```

### Order of Execution

(If not already exist: `clone`/`update` (if not exist) -> `extract` -> `install`)

`(plugin script loading)` -> `src` -> `multisrc` -> `load`


