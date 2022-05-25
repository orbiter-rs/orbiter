# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit
> Supports only macos, linux atm, Windows support planned

### Example ~/.orbiter.config.yml

```yaml
- id: neovim
  resource:
    repo: neovim/neovim
    is_release: true
  exec: '**/bin/nvim'
  load: 'export VISUAL=nvim; export EDITOR="$VISUAL"; alias vi="$VISUAL"'

- id: vim-plug
  resource:
    repo: junegunn/vim-plug
  install: 'mkdir -p ~/.local/share/nvim/site/autoload;  ln -sf "$PWD/plug.vim" ~/.local/share/nvim/site/autoload/plug.vim'

- id: starship
  resource:
    repo: starship/starship
    is_release: true
  exec: '**/starship'
  install: '**/starship init zsh > init-starship.zsh'
  src: 'init-starship.zsh'

- id: ripgrep
  resource:
    repo: BurntSushi/ripgrep
    is_release: true
  exec: '**/rg'

- id: zoxide
  resource:
    repo: ajeetdsouza/zoxide
    is_release: true
  exec: '**/zoxide'
  install: '**/zoxide init zsh > init-zoxide.zsh'
  src: 'init-zoxide.zsh'
  load: 'alias cd=z'

- id: fd
  resource:
    repo: sharkdp/fd
    is_release: true
  exec: '**/fd'
  load: "alias find='fd'"

- id: gitui
  resource:
    repo: extrawurst/gitui
    is_release: true
  exec: '**/gitui'

- id: delta
  resource:
    repo: dandavison/delta
    is_release: true
  exec: '**/delta'
  install: |
    git config --global pager.diff delta
    git config --global pager.log delta
    git config --global pager.reflog delta
    git config --global pager.show delta
    git config --global interactive.diffFilter "delta --color-only --features=interactive"
    git config --global delta.features decorations
    git config --global delta.interactive.keep-plus-minus-markers false
    git config --global delta.decorations.commit-decoration-style "blue ol"
    git config --global delta.decorations.commit-style raw
    git config --global delta.decorations.file-style omit
    git config --global delta.decorations.hunk-header-decoration-style blue box
    git config --global delta.decorations.hunk-header-file-style red
    git config --global delta.decorations.hunk-header-line-number-style "#067a00"
    git config --global delta.decorations.hunk-header-style "file line-number syntax"

- id: exa
  resource:
    repo: ogham/exa
    is_release: true
  exec: '**/exa'
  load: 'alias ls="exa --icons --color always"; alias ll=''ls -la'''

- id: bat
  resource:
    repo: sharkdp/bat
    is_release: true
  exec: '**/bat'
  load: 'alias cat=bat'

- id: bottom
  resource:
    repo: clementtsang/bottom
    is_release: true
  exec: '**/btm'
  load: 'alias top=btm'

- id: zellij
  resource:
    repo: zellij-org/zellij
    is_release: true
  exec: '**/zellij'

- id: direnv
  resource:
    repo: direnv/direnv
    is_release: true
  install: 'mv direnv* direnv; chmod +x ./direnv; ./direnv hook zsh > zhook.zsh'
  src: zhook.zsh
  load: export DIRENV_LOG_FORMAT=""
  exec: '**/direnv'

- id: gh
  resource:
    repo: cli/cli
    is_release: true
  exec: '**/gh'

- id: exercism
  resource:
    repo: exercism/cli
    is_release: true
  exec: '**/exercism'

- id: dprint
  resource:
    repo: dprint/dprint
    is_release: true
  exec: '**/dprint'

- id: fzf
  resource:
    repo: junegunn/fzf-bin
    is_release: true
  exec: '**/fzf'

- id: kubectl
  init: 'curl -L -s https://dl.k8s.io/release/stable.txt'
  resource:
    macos: https://storage.googleapis.com/kubernetes-release/release/{init}/bin/darwin/amd64/kubectl
    linux: https://storage.googleapis.com/kubernetes-release/release/{init}/bin/linux/amd64/kubectl
  install: 'chmod +x ./kubectl; ./kubectl completion zsh > zsh_completion.zsh'
  src: zsh_completion.zsh
  exec: kubectl

- id: minikube
  resource:
    macos: https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
    linux: https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
  install: 'chmod +x ./minikube; ./minikube completion zsh > zsh_completion.zsh'
  src: zsh_completion.zsh
  exec: minikube

- id: zsh-autosuggestions
  resource:
    repo: zsh-users/zsh-autosuggestions
  src: zsh-autosuggestions.zsh

- id: fast-syntax-highlighting
  resource:
    repo: z-shell/F-Sy-H
  src: f-sy-h.plugin.zsh
```

### Order of Execution

(If not already exist: `init` -> `clone`/`update` -> `extract` (supports auto extraction of "zip", "tar.gz", "deb") -> `install`)

`(plugin script loading)` -> `src` -> `multisrc` -> `load`
