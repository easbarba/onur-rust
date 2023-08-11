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

# Onur | Protocol

## Config

- cache latest bundled configs as ONUR_CONFIG/.cached
- bundle all files with syntax as

```json
"lang": "py",
"projects": {"name": "python", "branch": "dev", "https://github.com/python/python"}
```

- branch field defaults to Git, that is `master`
- check if empty
- check if config has properly syntax
- `url` to URI
- `name` as lowercase and \_ only

### Api

- parse all files

## Local

- parse json files and bundle as api sintaxe
- url to uri

### Single

- bundle file as config syntax
- consumes piped input

## Front

- consume array properly processed
- run action on array
- log actions and it's output
