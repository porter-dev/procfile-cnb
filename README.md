# Heroku Cloud Native Procfile Buildpack

[![CircleCI](https://circleci.com/gh/heroku/procfile-cnb/tree/main.svg?style=svg)](https://circleci.com/gh/heroku/procfile-cnb/tree/main)

This is a [Cloud Native Buildpack](https://buildpacks.io/) that replicates the behavior of
`Procfile` in [Heroku Buildpacks](https://devcenter.heroku.com/articles/buildpacks).

It is written in Rust using the Cloud Native Buildpack framework [libcnb.rs](https://github.com/Malax/libcnb.rs).

## Development

### Prerequisites

See [Development Environment Setup](https://github.com/Malax/libcnb.rs#development-environment-setup).

### Test

Run unit tests:

```
$ cargo test
```

Run integration tests:

```
$ cargo test -- --ignored
```

Or to run all of the tests at the same time:

```
$ cargo test -- --include-ignored
```

### Pack build example

```
$ cargo libcnb package \
&& pack build procfile_example_app --builder heroku/buildpacks:20 --buildpack target/buildpack/debug/heroku_procfile --path tests/fixtures/app_with_procfile --verbose \
&& docker run -it --rm --entrypoint worker procfile_example_app
```

```
$ pack inspect procfile_example_app | grep -A10 Processes
Processes:
  TYPE                 SHELL        COMMAND              ARGS
  web (default)        bash         node index.js
  worker               bash         while true; do echo 'lol'; sleep 2; done
```
