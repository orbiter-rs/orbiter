# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit

### Example config

```yaml

- id: ff-dev
  resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
  extract: "tar xvf *.tar.* --directory ../"
  exec: "**/firefox/firefox"
  install: "./GitAhead*.sh --include-subdir"
  launcher:
    name: firefox
    exec: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
    icon: firefox
    menu_name: Firefox

- id: tmux
  resource:
    repo: tmux/tmux
    ver: "3.1c"
  install: "./autogen.sh; ./configure; make;"
  exec: "**/tmux"


```

### Order of Execution

Order of execution of related hooks: `install`/`update` -> `extract` -> `make` -> `install` -> `(plugin script loading)` -> `src` -> `multisrc` -> `load`.

