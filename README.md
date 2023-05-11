## What is

CLI to handle env file like db

## Build

```
cargo build
```

## How to use

Default options.

- `--target-env`
  - You can overrite target env by passing this option
  - Defualt value is `.env`

### Get

```
envdb get <key>
```

### Scan

```
envdb scan <prefix>
```

### Put

```
envdb put <key> <val>
```
