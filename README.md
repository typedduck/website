# Website Scaffolding

![Build Status](https://img.shields.io/github/actions/workflow/status/typedduck/website/rust.yml)

This is a simple biased website scaffolding that I use for my projects. It uses
the following libraries and tools:

- [Axum](https://crates.io/crates/axum) for the web framework.
- [Askama](https://crates.io/crates/askama) for the template engine.
- [Htmx](https://htmx.org/) for the client-side interactivity.
- [Tailwind CSS](https://tailwindcss.com/) for the styling.

In the assets folder there are predefined fonts which are used in the Tailwind
CSS configuration. These are:

- [Inter](https://fonts.google.com/specimen/Inter) as the sans-serif font.
- [Merriweather](https://fonts.google.com/specimen/Merriweather) as the serif
  font.

The fonts and scripts are not loaded from a CDN but are included in the project
itself. This reduces the number of external dependencies and allows for the
website to be used offline.

The template `root.html` is the base template for all the pages. It includes all
the necessary CSS and JavaScript files and allows for the title and the
content to be overridden.

The `index.html` is the main page of the website and is the one that is shown
when the website is accessed. It is handled by the `home` function in the
`handler` module.

The `404.html` is the page that is shown when a page is not found. The handler
for this page is the `not_found` function in the `handler` module. It is
registered as the fallback route in the `main` function.

## Usage

To use this scaffolding, you can clone this repository and then modify the
`src/handler.rs` and `src/templates` files to suit your needs. You can also
modify the `assets` folder to include your own fonts and scripts.

The resulting server can be configured using a configuration file. The
configuration file is a TOML file that contains the following fields:

- `host`: The host on which the server will listen. Default is `0.0.0.0`.
- `port`: The port on which the server will listen. Default is `8080`.
- `log`: The log filter directive as defined by the `tracing-subscriber` crate.
  For more information on the filter directive, see the
  [documentation](https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/filter/struct.EnvFilter.tml).
  Default is `error`.
- `site.title`: The title of the website. There is no default value for this
  field.
- `site.language`: The language of the website. Default is `en`.
- `site.base`: The base URL of the website. This is used to generate the URLs
  for the pages. There is no default value for this field.
- `assets`: Is a table that contains an entry for every asset that is used in
  the website. The keys are the names of the assets and the values are tables
  with the following fields:
  - `route`: The route to the assets.
  - `path`: The path to the assets folder.

Configuration files can be defined as follows:

```toml
host = "0.0.0.0"
port = 8080
log = "info"

[site]
language = "en"
title = "My Website"
base = "http://localhost:8080"

[[assets]]
route = "/assets"
path = "assets"
```

The configuration file can be passed to the server using the `--config` flag.
For example:

```sh
$ cargo run --release -- --config config.toml
```

The server can also be configured using environment variables. The environment
variables that can be used are the same as the fields in the configuration file
but are prefixed with `WEBSITE_` and are in uppercase. For example:

```sh
$ WEBSITE_HOST="localhost" WEBSITE_PORT="8080" WEBSITE_LOG="info" cargo run --release
```

The prefix is defined as constants in the `lib.rs` file and can be changed
there. The constants are:

- `CONFIG_FILE`: The name of the default configuration file. It is set to
  `website.toml`.
- `CONFIG_PATHS`: The paths where the configuration file is searched for. It is
  set to `["."]`.
- `CONFIG_ENV_PREFIX`: The prefix for the environment variables. It is set to
  `WEBSITE`. The separator between the prefix and the field is `_`.
- `CONFIG_ENV_FILE`: The name of the environment variable that contains the
  path to the configuration file. It is set to `WEBSITE_CONFIG`.

## Docker

The server can be run in a Docker container. The Dockerfile is defined in the
repository and can be built using the following command:

```sh
$ docker build -t website .
```

The server can be run using the following command:

```sh
$ docker run -p 8080:8080 -it --rm website
```

The server can also be run using a configuration file. The configuration file
can be mounted to the container using the following command:

```sh
$ docker run -p 8080:8080 -v $(pwd)/config.toml:/config.toml -it --rm website --config /config.toml
```

This will override the default configuration file with the one that is mounted
to the container.

The container is built using the `rust:latest` image and the server is compiled
using the `--release` flag. This means that the server is optimized for
performance and the profile strips the debug information from the binary. The 
binary is a full self-contained statically linked binary that can be run on any
system that has the same architecture as the one that it was compiled on. The
container only contains the binary, the assets and the configuration file. This
means that the container is as small as possible and since no external
dependencies are used, the container provides minimal attack surface.

## License

This project is licensed under the MIT License or the Apache License 2.0, at
your option. For details, see the `LICENSE-MIT` and `LICENSE-APACHE` files for
more information.

## Support

If you like this project and want to support it, you can do so by:

- Giving it a star on GitHub.
- Sharing it with your friends.
- Contributing to the project by opening an issue or a pull request.
- Donating to the project by using the following links:
  - Bitcoin (Taproot): `bc1pqdck3v3r7sa4mgl0dztfzufa4xg66g8cpcgwvjax9rtx6mlxafdqcgw3g2`
  - Bitcoin (Segwit): `bc1qet2ypmsxtx6mc03329ft5a736fy906flm4c42a9d3e7mvu872tcs8myzs6`
  - [Patreon](https://www.patreon.com/typedduck)

Patreon supporters will be listed in the `SUPPORTERS.md` file.