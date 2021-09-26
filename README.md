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
docker build --tag <image-name> .
```

```shell
docker run --rm --init <image-name>
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
