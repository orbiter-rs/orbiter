# orbiter

> A cross-shell plugin and package manager, heavily inspired by zinit

### Order of Execution

Order of execution of related hooks: `install`/`update` -> `extract` -> `make` -> `install` -> `(plugin script loading)` -> `src` -> `multisrc` -> `load`.
