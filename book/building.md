# Building

For development, this repository contains a `Justfile` that allows you to use the
[just](https://just.systems/man/en/) task runner for common development tasks.

The available targets are:

| Name | Description |
| --- | --- |
| `test` | Run unit tests for all crates. |
| `style` | Check formatting and style. |
| `features` | Make sure all combinations of features compile. |
| `coverage` | Determine test coverage. |
| `semver` | Make sure that the versions are semver-compliant. |
| `publish` | Publishes the given crate. |
| `book` | Build book. |
| `docs` | Build code documentation. |

## Local

For local development, which is recommended, you need a recent installation of
[`rustup`](https://rustup.rs/). You may also be able to use `cargo` from your
package manager, if desired.

In addition, you will likely need some tooling. It is recommended that you
install these using `cargo`:

```bash
cargo install mdbook cargo-llvm-cov cargo-hack just cargo-semver-checks
```

Now, you should be able to run a `just` command, for example:

```bash
just checks
```

## Docker

This repository also contains a `Dockerfile`, which you can use to build a base
image the right version of Rust and the appropriate tooling. Generally, you do
not need to use this as Rust has fantastic backwards compatibility. However,
this does give you the exact same environment as the CI which can be valuable
sometimes.

If you prefix any of the `just` targets with `docker`, then it will run the
command within a temporary docker container that has the repository mapped in
read-only.  For example:

```bash
just docker checks
```

You have the choice of building the docker container locally with the
`docker-build` target, or pulling it from GitLab using the `docker-pull`
target. If you do not do either, it will pull it from GitLab by default.
