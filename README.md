<!--

 Copyright 2023 EAS Barbosa

     Licensed under the Apache License, Version 2.0(the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.

-->

# Onur

Easily manage multiple FLOSS repositories.

# Installation

[go](https://gitlab.com/easbarba/onur-go) | [java](https://gitlab.com/easbarba/onur-java) | [python](https://gitlab.com/easbarba/onur-python) | [php](https://gitlab.com/easbarba/onur-php) | [ruby](https://gitlab.com/easbarba/onur-ruby)

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

A TOML settings file may define the behavior of `onur`:

```toml
single-branch = true
quiet = true
depth = 1
```

## Options

Consult `onur --help` for more options.

## GNU Guix

In a system with GNU Guix binary installed, its even easier to grab all
dependencies: `guix shell`.

## TODO

Check the `TODO.md` for more information.

## LICENSE

[Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
