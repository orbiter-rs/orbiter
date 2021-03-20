# orbiter

> A cross-shell plugin manager for zsh and more.

### Order of Execution

Order of execution of related hooks: `install`/`update` -> `extract` -> `make` -> `install` -> `(plugin script loading)` -> `src` -> `multisrc` -> `load`.
