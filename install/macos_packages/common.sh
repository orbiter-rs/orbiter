#!/bin/bash

error() {
  echo "[ERROR]: $1"
  exit 1
}

orbiter_version() {
  orbiter_program_file="$1"
  # Check if this is a relative path: if so, prepend './' to it
  if [ "$1" = "${1#/}" ]; then
    orbiter_program_file="./$orbiter_program_file"
  fi

  # Try to get the version from three sources in the following order:
  #  - the ORBITER_VERSION envar (usually set by the CI)
  #  - Running the binary file
  #  - By cutting out the first version tag in Cargo.toml
  # These get increasingly fragile as we go down the list---ideally CI should
  # always run with ORBITER_VERSION set to avoid issues in determining version.
  if [ "$ORBITER_VERSION" != "" ]; then
    echo "$ORBITER_VERSION" | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
  elif "$orbiter_program_file" -V >/dev/null 2>&1; then
    "$orbiter_program_file" -V 2> /dev/null | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
  else
    pushd "$(git rev-parse --show-toplevel)" &> /dev/null || true
    grep '^version = \"\(.*\)\"' Cargo.toml | head -n 1 | cut -f 2 -d '"' | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
    popd &> /dev/null || true
  fi
}
