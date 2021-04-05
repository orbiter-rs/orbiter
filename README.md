# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit

### Example config

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

- id: edge-dev
  resource: https://go.microsoft.com/fwlink/?linkid=2124602
  extract: "ar xv *.deb; tar xvf data.tar.*"
  exec: "**/msedge-dev/msedge"
  launcher:
    name: edge-dev
    exec: "env LIBVA_DRIVER_NAME=iHD $(readlink -f msedge-dev/msedge) --enable-features=UseOzonePlatform --ozone-platform=wayland %U"
    icon: microsoft-edge-dev
    menu_name: edge-dev

- id: tmux
  resource:
    repo: tmux/tmux
    ver: "3.1c"
  install: "./autogen.sh; ./configure; make;"
  exec: "**/tmux"

- id: neovim
  resource:
    repo: neovim/neovim
    binary_pattern: "*.tar.gz"
  extract: "tar xvf *.tar.*"
  exec: "**/nvim*/bin/nvim"


```

### Order of Execution

Order of execution of related hooks: `install`/`update` -> `extract` -> `make` -> `install` -> `(plugin script loading)` -> `src` -> `multisrc` -> `load`.

