<!--
Onur is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Onur is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Onur. If not, see <https://www.gnu.org/licenses/>.
-->

# Onur

Easily manage multiple FLOSS repositories.

# Installation

[python](https://gitlab.com/easbarba/onur) |
[php](https://gitlab.com/easbarba/onur_php) | [ruby](https://gitlab.com/easbarba/qas.rb) | [go](https://gitlab.com/easbarba/qas_go) | [api-go](https://gitlab.com/easbarba/qas_api_go)

## Usage

`onur` consumes configuration in the following manners:

By default it looks for configuration files at `$XDG_CONFIG/onur` or in the
directory set in the `$ONUR_CONFIG_HOME` environment variable.

```shell
onur grab
onur archive nuxt,awesomewm,gitignore
```

## Configuration file

A `onur` single configuration file:

```json
[
  {
    "name": "awesomewm",
    "branch": "dev",
    "url": "https://github.com/awesomeWM/awesome"
  },
  {
    "name": "nuxt",
    "branch": "main",
    "url": "https://github.com/nuxt/framework"
  }
]
```

More examples of configuration files are at `examples`.

## Settings

`Onur` uses a sensible default setting, but its a
[MicroProfile-Config](https://microprofile.io/specifications/microprofile-config), so you can overwrite via system properties to maven or whatever.

`mvn exec:java -D exec.args="grab" -D single-branch=true -D quiet=false -D depth=5`

## Options

Consult `onur --help` for more options.

## GNU Guix

In a system with GNU Guix binary installed, its even easier to grab all
dependencies: `guix shell`.

## TODO

Check the `TODO.md` for more information.

## LICENSE

[GNU GENERAL PUBLIC LICENSE Version 3](https://www.gnu.org/licenses/gpl-3.0.en.html)
