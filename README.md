# orbiter

> A cross-shell plugin manager for zsh and more.

### Order of Execution

Order of execution of related Ice-mods: `init` -> `atpull!` -> `make'!!'` -> `mv` -> `cp` -> `make!` -> `atclone`/`atpull` -> `make` -> `(plugin script loading)` -> `src` -> `multisrc` -> `atload`.
