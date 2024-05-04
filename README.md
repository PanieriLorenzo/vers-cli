# Vers-CLI

> Simple CLI tools for bumping and parsing version numbers

![Crates.io Version](https://img.shields.io/crates/v/vers-cli)
![GitHub License](https://img.shields.io/github/license/PanieriLorenzo/vers-cli)

## Goals

- Simple: no complicated configuration, just a few well-designed features
- Composable: making full use of Unix pipes
- Language-independent: use this in any project, no matter the language

## Non-Goals

- Release automation: this is language-dependent and there's a million tools out there (especially for JavaScript).
- Conventional commits: parsing commits is messy and out of scope for this project, there are several good tools for this already.

## Examples

```bash
vers-cli 0.1.0 | vers-cli bump patch
# -> 0.1.1

# understands "v" prefix
vers-cli v1.0.0 | vers-cli bump major | vers-cli bump patch
# -> v2.0.1

# understands (some) pre-releases
vers-cli v1.0.0-rc.1 | vers-cli release
# -> v1.0.0
vers-cli v1.0.0-rc.1 | vers-cli bump rc
# -> v1.0.0-rc.2

# invariant over metadata
vers-cli v1.0.0-rc.1+500 | vers-cli release
# -> v1.0.0+500

# can bump metadata
vers-cli v1.0.0+500 | vers-cli bump build
# -> v1.0.0+501

# linting
vers-cli 01.0.0 --lint
# -> error: leading zeros not allowed
# -> hint: use `1.0.0` instead
```
