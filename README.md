# Vers-CLI

## Goals

- Simple: no complicated configuration, just a few well-designed features
- Composable: making full use of Unix pipes
- Language-independent: use this in any project, no matter the language

## Non-Goals

- Release automation: this is language-dependent and there's a million tools out there (especially for JavaScript).
- Conventional commits: parsing commits is messy and out of scope for this project, there are several good tools for this already.

## Examples

```bash
semcheck 0.1.0 | semcheck bump patch
# -> 0.1.1

# understands "v" prefix
semcheck v1.0.0 | semcheck bump major | semcheck bump patch
# -> v2.0.1

# understands (some) pre-releases
semcheck v1.0.0-rc.1 | semcheck release
# -> v1.0.0
semcheck v1.0.0-rc.1 | semcheck bump rc
# -> v1.0.0-rc.2

# invariant over metadata
semcheck v1.0.0-rc.1+500 | semcheck release
# -> v1.0.0+500

# can bump metadata
semcheck v1.0.0+500 | semcheck bump build
# -> v1.0.0+501

# linting
semcheck 01.0.0 --lint
# -> error: leading zeros not allowed
# -> hint: use `1.0.0` instead
```
