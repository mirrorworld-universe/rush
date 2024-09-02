# rush-cli

## Features

### `rush new <NAME>`

Creates new project

### `rush deploy <PATH>`

Deploy current project

Available Options:
    - `--dry-run`

### `rush storage`

Work with storage

Available Options:
    - `start`
    - `view`
        - `blueprint`
        - `world` `<ADDRESS>`
        - `entity` `<ENTITY_NAME>`
        - `instance` `<WORLD>` `<REGION>` `<ENTITY>`
    - `reset`

### `rush config`

SET or GET configurations

Available Options:
    - `set`
        - `rpc` `<MONIKER_OR_CUSTOM>`
        - `ws` `<MONIKER_OR_CUSTOM>`
        - `keypair` `<FILE>`
        - `blueprint` `<FILE_OR_DIRECTORY>`
        - `storage` `memory` | `test-validator` `onchain`
    - `get`
