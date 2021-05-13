# Sway-Clock

A trivial clock implementation, which might be used for
[Sway](https://swaywm.org/).

## Installation

`cargo install --prefix .`

## Configuration

Put the following in `~/.config/sway/config`:

```
bar {
    # ...

    status_command sway-clock '"%Y-%m-%d %l:%M:%S %p"'

    # ...
}
```

## (Optional) Run in Docker container ;-)

```shell
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```

```shell
docker build --tag scratch-test .
```

```shell
docker run --rm --init <image-name>
```
