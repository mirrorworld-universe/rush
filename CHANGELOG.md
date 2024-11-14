# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Nightly

Nightly is tracked by the `main` branch.

## Stable

Stable is tracked by the `stable` branch

## How to add to this Changelog

1. Output the pretty-formatted Changelog by running the following command below

```bash
git log --pretty="- %s"
```

2. Copy output and paste in its respective release channel section

3. Correct any commit message that does not follow the [Convential Commit](https://www.conventionalcommits.org/en/v1.0.0/) standard

4. Feel free to add more context for commits that require more information like so below:

```
- feat: CHANGELOG
- feat: RELEASE guide
    - *Insert more context regarding the commit here*
- chore: Change email in SECURITY
- fix: Change CONTRIBUTING hirearchy
- feat: Add node_modules in gitignore
```
