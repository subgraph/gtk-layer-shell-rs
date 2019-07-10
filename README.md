# Rust bindings for gtk-layer-shell

This repository provides Rust bindings for the https://github.com/wmww/gtk-layer-shell
library. To use these bindings, it is assumed that you have built and installed
`gtk-layer-shell` and its dependencies.

## Usage

Examples can be found in the `examples/` directory. To run an example:

```bash
$ cargo run --example example
```

## Regenerating bindings

Sometimes the `.gir` file for gtk-layer-shell will need to be updated with
new versions, API changes, etc. The bindings will need to be regenerated when
the new file is added.

To regenerate the bindings:

```bash
$ make gir
```

Afterwards, ensure that building and running the examples still works.

## Greetz

@wmww for `gtk-layer-shell` in the first place

