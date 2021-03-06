# example-cli

![Build badge](https://github.com/cmsd2/example-cli/workflows/Rust/badge.svg)

## Workflow

### Trunk-based dev

1. Make changes locally
2. Run tests with cargo test
3. Commit and push to master
4. CI tests and builds prereleases for each platform

### Pull-request flow

1. Make changes locally
2. Run tests with cargo test
3. Commit and push to branch or fork
4. Review CI results
5. Make pull request
6. Review CI results and merge

## Release process

1. Update version in toml files
2. Commit to master
3. Tag with v* version
4. Push tags
5. CI tests and builds releases for each platform
6. CI creates a github release and attaches binaries
